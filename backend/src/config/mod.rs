use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WxOffs {
    #[serde(flatten)]
    pub versions: HashMap<String, Vec<u32>>,
}

pub fn load_wx_offs() -> Result<HashMap<String, Vec<u32>>> {
    let config_path = get_config_path()?;
    
    if !config_path.exists() {
        // 创建默认配置
        let default_offs = create_default_offs();
        save_wx_offs(&default_offs)?;
        return Ok(default_offs);
    }

    let content = fs::read_to_string(&config_path)
        .with_context(|| format!("Failed to read config file: {:?}", config_path))?;
    
    let wx_offs: WxOffs = serde_json::from_str(&content)
        .with_context(|| "Failed to parse WX_OFFS.json")?;
    
    Ok(wx_offs.versions)
}

pub fn save_wx_offs(offs: &HashMap<String, Vec<u32>>) -> Result<()> {
    let config_path = get_config_path()?;
    
    // 确保目录存在
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    let wx_offs = WxOffs {
        versions: offs.clone(),
    };
    
    let content = serde_json::to_string_pretty(&wx_offs)?;
    fs::write(&config_path, content)?;
    
    Ok(())
}

fn get_config_path() -> Result<PathBuf> {
    // 优先使用项目目录下的配置文件
    if let Ok(mut path) = std::env::current_dir() {
        let project_config = path.join("backend").join("config").join("wx_offs.json");
        if project_config.exists() {
            return Ok(project_config);
        }
    }
    
    // 否则使用可执行文件目录
    let mut path = std::env::current_exe()?;
    path.pop(); // 移除可执行文件名
    path.push("config");
    path.push("wx_offs.json");
    Ok(path)
}

fn create_default_offs() -> HashMap<String, Vec<u32>> {
    // 从原项目复制一些基础版本偏移量
    let mut offs = HashMap::new();
    
    // 示例：3.9.12.55
    offs.insert("3.9.12.55".to_string(), vec![94550988, 94552544, 94551016, 0, 94552480]);
    
    // 可以在这里添加更多版本，或者从原项目导入
    offs
}

