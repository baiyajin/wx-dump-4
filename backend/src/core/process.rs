use crate::utils::Result;
use anyhow::Context;
use std::ffi::CString;
use windows::{
    core::PCSTR,
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

pub struct ProcessManager {
    pub pid: u32,
    pub handle: HANDLE,
}

impl ProcessManager {
    pub fn open(pid: u32) -> Result<Self> {
        let handle = unsafe {
            OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid)
                .with_context(|| format!("Failed to open process {}", pid))?
        };

        Ok(Self { pid, handle })
    }

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
                    let exe_name = CString::from_raw(pe32.szExeFile.as_ptr() as *mut i8);
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

