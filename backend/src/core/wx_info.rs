use crate::core::{
    file_version::{get_file_version_info, get_process_exe_path},
    memory::MemoryManager,
    memory_map::MemoryMap,
    process::ProcessManager,
    version,
};
use crate::utils::Result;
use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use winreg::enums::*;
use winreg::RegKey;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeChatInfo {
    pub pid: u32,
    pub version: String,
    pub account: Option<String>,
    pub mobile: Option<String>,
    pub nickname: Option<String>,
    pub mail: Option<String>,
    pub wxid: Option<String>,
    pub key: Option<String>,
    pub wx_dir: Option<String>,
}

pub fn get_wx_info(wx_offs: &HashMap<String, Vec<u32>>) -> Result<Vec<WeChatInfo>> {
    // 查找所有WeChat进程
    let pids = ProcessManager::find_by_name("WeChat.exe")?;

    if pids.is_empty() {
        return Err(crate::utils::AppError::WeChatNotFound.into());
    }

    let mut results = Vec::new();

    for pid in pids {
        match get_info_details(pid, wx_offs) {
            Ok(info) => results.push(info),
            Err(e) => {
                tracing::warn!("Failed to get info for PID {}: {}", pid, e);
            }
        }
    }

    Ok(results)
}

pub fn get_info_details(pid: u32, wx_offs: &HashMap<String, Vec<u32>>) -> Result<WeChatInfo> {
    let process = ProcessManager::open(pid)?;
    let memory = MemoryManager::new(process.handle);

    // 先尝试从WeChatWin.dll获取真实版本
    let mut version = get_wechat_version(pid)?;
    
    // 从内存映射中查找WeChatWin.dll并获取真实版本
    if let Ok(Some(wechatwin_path)) = MemoryMap::get_wechatwin_path(pid) {
        if let Ok(dll_version) = get_file_version_info(std::path::Path::new(&wechatwin_path)) {
            version = dll_version;
        }
    }
    
    let addr_len = version::detect_address_len(&version);

    // 获取WeChatWin.dll基址（优先从内存映射获取）
    let wechat_base_address = match MemoryMap::get_wechatwin_base_address(pid) {
        Ok(addr) => addr,
        Err(_) => get_wechat_base_address(&memory)?,
    };

    let mut info = WeChatInfo {
        pid,
        version: version.clone(),
        account: None,
        mobile: None,
        nickname: None,
        mail: None,
        wxid: None,
        key: None,
        wx_dir: None,
    };

    // 根据偏移量获取信息
    if let Some(bias_list) = version::get_version_offset(&version, wx_offs) {
        if bias_list.len() >= 5 {
            let name_bias = bias_list[0] as usize;
            let account_bias = bias_list[1] as usize;
            let mobile_bias = bias_list[2] as usize;
            let mail_bias = bias_list[3] as usize;
            let key_bias = bias_list[4] as usize;

            if name_bias > 0 {
                info.nickname = read_info_name(&memory, wechat_base_address + name_bias, addr_len)?;
            }
            if account_bias > 0 {
                info.account = memory.read_string(wechat_base_address + account_bias, 32)?;
            }
            if mobile_bias > 0 {
                info.mobile = memory.read_string(wechat_base_address + mobile_bias, 64)?;
            }
            if mail_bias > 0 {
                info.mail = memory.read_string(wechat_base_address + mail_bias, 64)?;
            }
            if key_bias > 0 {
                info.key = read_key_by_offs(&memory, wechat_base_address + key_bias, addr_len)?;
            }
        }
    }

    // 获取wxid
    info.wxid = get_wxid(&memory)?;

    // 获取微信目录
    if let Some(ref wxid) = info.wxid {
        info.wx_dir = get_wx_dir_by_reg(wxid);
    }

    Ok(info)
}

fn read_info_name(
    memory: &MemoryManager,
    address: usize,
    addr_len: usize,
) -> Result<Option<String>> {
    // 先读取指针
    if let Ok(ptr) = memory.read_pointer(address, addr_len) {
        if ptr > 0 {
            if let Ok(Some(name)) = memory.read_string(ptr, 64) {
                return Ok(Some(name));
            }
        }
    }

    // 如果指针读取失败，直接读取字符串
    memory.read_string(address, 64)
}

fn read_key_by_offs(
    memory: &MemoryManager,
    address: usize,
    addr_len: usize,
) -> Result<Option<String>> {
    // 读取指针指向的地址
    let key_ptr = memory.read_pointer(address, addr_len)?;
    if key_ptr == 0 {
        return Ok(None);
    }

    // 读取32字节的key
    let key_bytes = memory.read_memory(key_ptr, 32)?;
    Some(hex::encode(key_bytes)).ok_or_else(|| anyhow::anyhow!("Failed to encode key").into())
}

fn get_wxid(memory: &MemoryManager) -> Result<Option<String>> {
    // 搜索内存中的wxid模式
    let pattern = b"\\Msg\\FTSContact";
    let results = memory.search_memory(pattern, 0, 0x7FFFFFFFFFFFFFFF, 100)?;

    if results.is_empty() {
        return Ok(None);
    }

    // 从找到的地址提取wxid
    for addr in results {
        if let Ok(Some(path)) = memory.read_string(addr.saturating_sub(100), 200) {
            if let Some(wxid) = extract_wxid_from_path(&path) {
                return Ok(Some(wxid));
            }
        }
    }

    Ok(None)
}

fn extract_wxid_from_path(path: &str) -> Option<String> {
    // 从路径中提取wxid，例如: C:\Users\...\WeChat Files\wxid_xxxxx\Msg\FTSContact
    if let Some(pos) = path.find("\\Msg\\") {
        let wxid_part = &path[..pos];
        if let Some(wxid) = wxid_part.split('\\').last() {
            if wxid.starts_with("wxid_") {
                return Some(wxid.to_string());
            }
        }
    }
    None
}

fn get_wechat_base_address(memory: &MemoryManager) -> Result<usize> {
    let maps = memory.get_memory_maps()?;
    
    for map in maps {
        if let Some(ref file_name) = map.file_name {
            if file_name.contains("WeChatWin.dll") {
                return Ok(map.base_address);
            }
        }
    }

    Err(anyhow::anyhow!("WeChatWin.dll not found").into())
}

fn get_wechat_version(pid: u32) -> Result<String> {
    // 从进程路径获取版本信息
    let exe_path = get_process_exe_path(pid)?;
    let path = PathBuf::from(&exe_path);
    
    if path.exists() {
        get_file_version_info(&path)
    } else {
        // 如果文件不存在，尝试从内存映射中获取
        Err(anyhow::anyhow!("Cannot get WeChat version: file not found").into())
    }
}

fn get_wx_dir_by_reg(wxid: &str) -> Option<String> {
    // 从注册表读取微信文件路径
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    
    // 尝试从注册表读取
    if let Ok(key) = hkcu.open_subkey(r"Software\Tencent\WeChat") {
        if let Ok(path) = key.get_value::<String, _>("FileSavePath") {
            let wx_dir = PathBuf::from(path).join("WeChat Files");
            let wxid_dir = wx_dir.join(wxid);
            if wxid_dir.exists() {
                return wxid_dir.to_str().map(|s| s.to_string());
            }
        }
    }

    // 从环境变量获取文档路径
    if let Ok(user_profile) = std::env::var("USERPROFILE") {
        let wx_dir = PathBuf::from(user_profile)
            .join("Documents")
            .join("WeChat Files")
            .join(wxid);
        if wx_dir.exists() {
            return wx_dir.to_str().map(|s| s.to_string());
        }
    }

    None
}

