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

