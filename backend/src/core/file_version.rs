use crate::utils::Result;
use anyhow::Context;
use std::path::Path;
use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::HANDLE,
        System::{
            Diagnostics::ToolHelp::GetModuleFileNameExW,
            Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ},
        },
    },
};

pub fn get_file_version_info(file_path: &Path) -> Result<String> {
    use windows::Win32::System::SystemServices::*;
    
    let path_str: Vec<u16> = file_path
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid file path"))?
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();

    unsafe {
        let size = GetFileVersionInfoSizeW(
            PCWSTR::from_raw(path_str.as_ptr() as *const u16),
            None,
        );

        if size == 0 {
            return Err(anyhow::anyhow!("Failed to get version info size").into());
        }

        let mut buffer = vec![0u8; size as usize];
        if !GetFileVersionInfoW(
            PCWSTR::from_raw(path_str.as_ptr() as *const u16),
            0,
            size,
            buffer.as_mut_ptr() as *mut _,
        )
        .as_bool()
        {
            return Err(anyhow::anyhow!("Failed to get version info").into());
        }

        let mut buffer_ptr: *mut std::ffi::c_void = std::ptr::null_mut();
        let mut len: u32 = 0;

        let sub_block: Vec<u16> = "\\\0".encode_utf16().collect();
        if !VerQueryValueW(
            buffer.as_ptr() as *const _,
            PCWSTR::from_raw(sub_block.as_ptr() as *const u16),
            &mut buffer_ptr,
            &mut len,
        )
        .as_bool()
        {
            return Err(anyhow::anyhow!("Failed to query version value").into());
        }

        let ffi = &*(buffer_ptr as *const VS_FIXEDFILEINFO);

        if ffi.dwSignature != 0xFEEF04BD {
            return Err(anyhow::anyhow!("Invalid version signature").into());
        }

        let version = format!(
            "{}.{}.{}.{}",
            (ffi.dwFileVersionMS >> 16) & 0xFFFF,
            ffi.dwFileVersionMS & 0xFFFF,
            (ffi.dwFileVersionLS >> 16) & 0xFFFF,
            ffi.dwFileVersionLS & 0xFFFF
        );

        Ok(version)
    }
}

pub fn get_process_exe_path(pid: u32) -> Result<String> {
    let handle = unsafe {
        OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid)
            .with_context(|| format!("Failed to open process {}", pid))?
    };

    unsafe {
        let mut buffer = vec![0u16; 260];
        let len = GetModuleFileNameExW(handle, None, &mut buffer);

        if len == 0 {
            return Err(anyhow::anyhow!("Failed to get module file name").into());
        }

        buffer.truncate(len as usize);
        let path = String::from_utf16(&buffer)
            .with_context(|| "Failed to convert path to UTF-8")?;

        Ok(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_get_file_version_info() {
        // 测试获取文件版本信息
        // 注意：这个测试需要实际存在的文件
        // 可以测试系统文件如notepad.exe
        let system32 = std::env::var("WINDIR")
            .map(|w| PathBuf::from(w).join("System32").join("notepad.exe"))
            .ok();
        
        if let Some(path) = system32 {
            if path.exists() {
                let result = get_file_version_info(&path);
                // 如果文件存在，应该能获取版本信息
                if result.is_ok() {
                    println!("Notepad version: {}", result.unwrap());
                }
            }
        }
    }

    #[test]
    fn test_get_file_version_info_invalid() {
        // 测试无效文件路径
        let invalid_path = PathBuf::from("C:\\NonExistentFile.exe");
        let result = get_file_version_info(&invalid_path);
        assert!(result.is_err());
    }
}

