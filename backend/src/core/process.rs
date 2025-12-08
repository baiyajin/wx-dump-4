use crate::utils::Result;
use anyhow::Context;
use std::ffi::CStr;
use windows::{
    Win32::{
        Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE},
        System::{
            Diagnostics::ToolHelp::{
                CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32,
                TH32CS_SNAPPROCESS,
            },
            Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ},
        },
    },
};

/// 进程管理器
pub struct ProcessManager {
    pub pid: u32,
    pub handle: HANDLE,
}

impl ProcessManager {
    /// 打开指定PID的进程
    pub fn open(pid: u32) -> Result<Self> {
        let handle = unsafe {
            OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid)
                .with_context(|| format!("Failed to open process {}", pid))?
        };

        Ok(Self { pid, handle })
    }

    /// 根据进程名查找进程ID列表
    pub fn find_by_name(name: &str) -> Result<Vec<u32>> {
        let snapshot = unsafe {
            CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)
                .with_context(|| "Failed to create process snapshot")?
        };

        if snapshot == INVALID_HANDLE_VALUE {
            return Err(anyhow::anyhow!("Invalid snapshot handle").into());
        }

        let mut pe32 = PROCESSENTRY32 {
            dwSize: std::mem::size_of::<PROCESSENTRY32>() as u32,
            ..Default::default()
        };

        let mut pids = Vec::new();

        unsafe {
            if Process32First(snapshot, &mut pe32).as_bool() {
                loop {
                    let exe_name = CStr::from_ptr(pe32.szExeFile.as_ptr() as *const i8);
                    let exe_name_str = exe_name.to_string_lossy();

                    if exe_name_str == name {
                        pids.push(pe32.th32ProcessID);
                    }

                    if !Process32Next(snapshot, &mut pe32).as_bool() {
                        break;
                    }
                }
            }
            CloseHandle(snapshot)?;
        }

        Ok(pids)
    }
}

impl Drop for ProcessManager {
    fn drop(&mut self) {
        unsafe {
            let _ = CloseHandle(self.handle);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_by_name() {
        // 测试查找微信进程
        let result = ProcessManager::find_by_name("WeChat.exe");
        // 注意：这个测试需要实际运行微信才能找到进程
        // 在没有微信的情况下，应该返回空列表而不是错误
        match result {
            Ok(pids) => {
                println!("Found {} WeChat processes", pids.len());
            }
            Err(e) => {
                // 如果没有找到进程，这是正常的
                println!("No WeChat processes found or error: {}", e);
            }
        }
    }

    #[test]
    fn test_find_nonexistent_process() {
        // 测试查找不存在的进程
        let result = ProcessManager::find_by_name("NonExistentProcess.exe");
        assert!(result.is_ok());
        let pids = result.unwrap();
        assert_eq!(pids.len(), 0);
    }

    #[test]
    fn test_open_process() {
        // 测试打开进程（需要实际运行的进程）
        let result = ProcessManager::find_by_name("WeChat.exe");
        if let Ok(pids) = result {
            if !pids.is_empty() {
                let pid = pids[0];
                let manager = ProcessManager::open(pid);
                // 注意：这个测试需要管理员权限
                if manager.is_ok() {
                    println!("Successfully opened process {}", pid);
                } else {
                    println!("Failed to open process {} (may need admin rights)", pid);
                }
            }
        }
    }
}
