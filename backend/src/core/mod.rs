pub mod process;
pub mod memory;
pub mod memory_map;
pub mod decryption;
pub mod version;
pub mod version_detection;
pub mod wx_info;
pub mod file_version;

pub use process::ProcessManager;
pub use memory::MemoryManager;
pub use wx_info::{WeChatInfo, get_wx_info};
pub use file_version::{get_file_version_info, get_process_exe_path};

