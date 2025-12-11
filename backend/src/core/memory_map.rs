use crate::utils::Result;
use anyhow::Context;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use windows::Win32::{
    System::{
        Diagnostics::ToolHelp::{
            CreateToolhelp32Snapshot, Module32FirstW, Module32NextW, MODULEENTRY32W,
            TH32CS_SNAPMODULE, TH32CS_SNAPMODULE32,
        },
    },
};

#[derive(Debug, Clone)]
pub struct ModuleInfo {
    pub base_address: usize,
    pub size: usize,
    pub file_name: String,
}

pub struct MemoryMap;

impl MemoryMap {
    /// 获取进程的所有模块信息
    pub fn get_modules(pid: u32) -> Result<Vec<ModuleInfo>> {
        let snapshot = unsafe {
            CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, pid)
                .with_context(|| format!("Failed to create snapshot for PID {}", pid))?
        };

        let mut modules = Vec::new();
        let mut entry = MODULEENTRY32W {
            dwSize: std::mem::size_of::<MODULEENTRY32W>() as u32,
            ..Default::default()
        };

        unsafe {
            if Module32FirstW(snapshot, &mut entry).as_bool() {
                loop {
                    let file_name = Self::wide_string_to_string(&entry.szModule)?;
                    let file_path = Self::wide_string_to_string(&entry.szExePath)?;

                    modules.push(ModuleInfo {
                        base_address: entry.modBaseAddr as usize,
                        size: entry.modBaseSize as usize,
                        file_name,
                    });

                    if !Module32NextW(snapshot, &mut entry).as_bool() {
                        break;
                    }
                }
            }
        }

        Ok(modules)
    }

    /// 查找WeChatWin.dll模块
    pub fn find_wechatwin_dll(pid: u32) -> Result<Option<ModuleInfo>> {
        let modules = Self::get_modules(pid)?;

        for module in modules {
            if module.file_name.to_lowercase().contains("wechatwin.dll") {
                return Ok(Some(module));
            }
        }

        Ok(None)
    }

    /// 获取WeChatWin.dll的基址
    pub fn get_wechatwin_base_address(pid: u32) -> Result<usize> {
        if let Some(module) = Self::find_wechatwin_dll(pid)? {
            Ok(module.base_address)
        } else {
            Err(anyhow::anyhow!("WeChatWin.dll not found").into())
        }
    }

    /// 获取WeChatWin.dll的文件路径
    pub fn get_wechatwin_path(pid: u32) -> Result<Option<String>> {
        let snapshot = unsafe {
            CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, pid)
                .with_context(|| format!("Failed to create snapshot for PID {}", pid))?
        };

        let mut entry = MODULEENTRY32W {
            dwSize: std::mem::size_of::<MODULEENTRY32W>() as u32,
            ..Default::default()
        };

        unsafe {
            if Module32FirstW(snapshot, &mut entry).as_bool() {
                loop {
                    let file_name = Self::wide_string_to_string(&entry.szModule)?;
                    
                    if file_name.to_lowercase().contains("wechatwin.dll") {
                        let file_path = Self::wide_string_to_string(&entry.szExePath)?;
                        return Ok(Some(file_path));
                    }

                    if !Module32NextW(snapshot, &mut entry).as_bool() {
                        break;
                    }
                }
            }
        }

        Ok(None)
    }

    fn wide_string_to_string(wide: &[u16]) -> Result<String> {
        let end = wide.iter().position(|&c| c == 0).unwrap_or(wide.len());
        let string = OsString::from_wide(&wide[..end])
            .to_string_lossy()
            .to_string();
        Ok(string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_modules() {
        // 测试获取当前进程的模块列表
        let current_pid = std::process::id();
        let result = MemoryMap::get_modules(current_pid);
        
        // 应该能获取到模块列表
        if let Ok(modules) = result {
            assert!(!modules.is_empty());
            println!("Found {} modules in current process", modules.len());
        }
    }

    #[test]
    fn test_find_wechatwin_dll() {
        // 测试查找WeChatWin.dll（需要微信进程运行）
        let pids = crate::core::process::ProcessManager::find_by_name("WeChat.exe");
        
        if let Ok(pids) = pids {
            if !pids.is_empty() {
                let pid = pids[0];
                let result = MemoryMap::find_wechatwin_dll(pid);
                // 如果微信运行，应该能找到WeChatWin.dll
                if let Ok(Some(module)) = result {
                    println!("Found WeChatWin.dll at 0x{:x}", module.base_address);
                }
            }
        }
    }

    #[test]
    fn test_get_wechatwin_base_address() {
        // 测试获取WeChatWin.dll基址
        let pids = crate::core::process::ProcessManager::find_by_name("WeChat.exe");
        
        if let Ok(pids) = pids {
            if !pids.is_empty() {
                let pid = pids[0];
                let result = MemoryMap::get_wechatwin_base_address(pid);
                if result.is_ok() {
                    println!("WeChatWin.dll base address: 0x{:x}", result.unwrap());
                }
            }
        }
    }
}

