use crate::utils::Result;
use anyhow::Context;
use windows::{
    Win32::{
        Foundation::HANDLE,
        System::{
            Memory::{
                MEMORY_BASIC_INFORMATION, MEM_COMMIT, PAGE_EXECUTE, PAGE_EXECUTE_READ,
                PAGE_EXECUTE_READWRITE, PAGE_READONLY, PAGE_READWRITE, VirtualQueryEx,
            },
            ProcessStatus::GetMappedFileNameW,
            Threading::ReadProcessMemory,
        },
    },
};

pub struct MemoryManager {
    handle: HANDLE,
}

impl MemoryManager {
    pub fn new(handle: HANDLE) -> Self {
        Self { handle }
    }

    pub fn read_memory(&self, address: usize, size: usize) -> Result<Vec<u8>> {
        let mut buffer = vec![0u8; size];
        let mut bytes_read = 0;

        unsafe {
            ReadProcessMemory(
                self.handle,
                address as *const _,
                buffer.as_mut_ptr() as *mut _,
                size,
                Some(&mut bytes_read),
            )
            .ok()
            .with_context(|| format!("Failed to read memory at 0x{:x}", address))?;
        }

        buffer.truncate(bytes_read);
        Ok(buffer)
    }

    pub fn read_string(&self, address: usize, max_size: usize) -> Result<Option<String>> {
        let data = self.read_memory(address, max_size)?;
        
        // 查找null终止符
        if let Some(null_pos) = data.iter().position(|&b| b == 0) {
            let string_data = &data[..null_pos];
            String::from_utf8(string_data.to_vec())
                .ok()
                .map(|s| Some(s.trim().to_string()))
                .unwrap_or(None)
        } else {
            String::from_utf8(data)
                .ok()
                .map(|s| Some(s.trim().to_string()))
                .unwrap_or(None)
        }
        .ok_or_else(|| anyhow::anyhow!("Invalid UTF-8 string").into())
    }

    pub fn read_pointer(&self, address: usize, addr_len: usize) -> Result<usize> {
        let data = self.read_memory(address, addr_len)?;
        
        if addr_len == 8 {
            Ok(u64::from_le_bytes([
                data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
            ]) as usize)
        } else {
            Ok(u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize)
        }
    }

    pub fn search_memory(
        &self,
        pattern: &[u8],
        start_address: usize,
        end_address: usize,
        max_results: usize,
    ) -> Result<Vec<usize>> {
        let mut results = Vec::new();
        let mut address = start_address;

        while address < end_address && results.len() < max_results {
            let mbi = self.query_memory(address)?;
            
            if mbi.State != MEM_COMMIT {
                address += mbi.RegionSize;
                continue;
            }

            let allowed_protections = [
                PAGE_EXECUTE,
                PAGE_EXECUTE_READ,
                PAGE_EXECUTE_READWRITE,
                PAGE_READWRITE,
                PAGE_READONLY,
            ];

            if !allowed_protections.contains(&mbi.Protect) {
                address += mbi.RegionSize;
                continue;
            }

            // 读取内存区域
            if let Ok(data) = self.read_memory(address, mbi.RegionSize) {
                // 搜索模式
                for (offset, window) in data.windows(pattern.len()).enumerate() {
                    if window == pattern {
                        results.push(address + offset);
                        if results.len() >= max_results {
                            break;
                        }
                    }
                }
            }

            address += mbi.RegionSize;
        }

        Ok(results)
    }

    fn query_memory(&self, address: usize) -> Result<MEMORY_BASIC_INFORMATION> {
        let mut mbi = MEMORY_BASIC_INFORMATION::default();
        
        unsafe {
            VirtualQueryEx(
                self.handle,
                Some(address as *const _),
                &mut mbi,
                std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
            )
            .ok()
            .with_context(|| format!("Failed to query memory at 0x{:x}", address))?;
        }

        Ok(mbi)
    }

    pub fn get_memory_maps(&self) -> Result<Vec<MemoryMap>> {
        let mut maps = Vec::new();
        let mut address = 0usize;
        let max_address = 0x7FFFFFFFFFFFFFFFusize;

        while address < max_address {
            let mbi = match self.query_memory(address) {
                Ok(mbi) => mbi,
                Err(_) => break,
            };

            let file_name = unsafe {
                let mut buffer = vec![0u16; 260];
                let len = GetMappedFileNameW(self.handle, address as *const _, &mut buffer);
                if len > 0 {
                    String::from_utf16(&buffer[..len as usize]).ok()
                } else {
                    None
                }
            };

            maps.push(MemoryMap {
                base_address: mbi.BaseAddress as usize,
                region_size: mbi.RegionSize,
                state: mbi.State,
                protect: mbi.Protect,
                file_name,
            });

            address += mbi.RegionSize;
        }

        Ok(maps)
    }
}

#[derive(Debug, Clone)]
pub struct MemoryMap {
    pub base_address: usize,
    pub region_size: usize,
    pub state: u32,
    pub protect: u32,
    pub file_name: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::process::ProcessManager;

    #[test]
    fn test_read_memory() {
        // 测试读取内存（需要实际运行的进程）
        let pids = ProcessManager::find_by_name("WeChat.exe");
        
        if let Ok(pids) = pids {
            if !pids.is_empty() {
                let pid = pids[0];
                if let Ok(process) = ProcessManager::open(pid) {
                    let memory = MemoryManager::new(process.handle);
                    // 尝试读取一个有效的内存地址（需要根据实际情况调整）
                    // 这里只是测试接口，不保证地址有效
                    let result = memory.read_memory(0x1000000, 1024);
                    // 读取可能成功或失败，取决于地址是否有效
                    if result.is_ok() {
                        println!("Successfully read memory");
                    }
                }
            }
        }
    }

    #[test]
    fn test_read_string() {
        // 测试读取字符串
        let pids = ProcessManager::find_by_name("WeChat.exe");
        
        if let Ok(pids) = pids {
            if !pids.is_empty() {
                let pid = pids[0];
                if let Ok(process) = ProcessManager::open(pid) {
                    let memory = MemoryManager::new(process.handle);
                    // 测试读取字符串（需要有效的地址）
                    let result = memory.read_string(0x1000000, 256);
                    // 可能成功或失败
                    if result.is_ok() {
                        println!("Successfully read string");
                    }
                }
            }
        }
    }

    #[test]
    fn test_read_pointer() {
        // 测试读取指针
        let pids = ProcessManager::find_by_name("WeChat.exe");
        
        if let Ok(pids) = pids {
            if !pids.is_empty() {
                let pid = pids[0];
                if let Ok(process) = ProcessManager::open(pid) {
                    let memory = MemoryManager::new(process.handle);
                    // 测试读取64位指针
                    let result = memory.read_pointer(0x1000000, 8);
                    if result.is_ok() {
                        println!("Read pointer value: 0x{:x}", result.unwrap());
                    }
                }
            }
        }
    }
}

