use crate::utils::Result;
use anyhow::anyhow;
use std::path::Path;

/// 验证数据库路径
pub fn validate_db_path(path: &str) -> Result<()> {
    let path = Path::new(path);
    
    if !path.exists() {
        return Err(anyhow!("Database file does not exist: {}", path.display()).into());
    }
    
    if !path.is_file() {
        return Err(anyhow!("Path is not a file: {}", path.display()).into());
    }
    
    if path.extension().and_then(|s| s.to_str()) != Some("db") {
        return Err(anyhow!("File is not a database file (.db): {}", path.display()).into());
    }
    
    Ok(())
}

/// 验证密钥格式
pub fn validate_key(key: &str) -> Result<()> {
    if key.is_empty() {
        return Err(anyhow!("Key cannot be empty").into());
    }
    
    // 密钥应该是64个十六进制字符（32字节）
    if key.len() != 64 {
        return Err(anyhow!("Key must be 64 hexadecimal characters (32 bytes)").into());
    }
    
    if !key.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(anyhow!("Key must contain only hexadecimal characters").into());
    }
    
    Ok(())
}

/// 验证时间范围
pub fn validate_time_range(start_time: Option<i64>, end_time: Option<i64>) -> Result<()> {
    if let (Some(start), Some(end)) = (start_time, end_time) {
        if start > end {
            return Err(anyhow!("Start time must be less than or equal to end time").into());
        }
    }
    
    Ok(())
}

/// 验证分页参数
pub fn validate_pagination(start: i64, limit: i64) -> Result<()> {
    if start < 0 {
        return Err(anyhow!("Start index must be non-negative").into());
    }
    
    if limit <= 0 {
        return Err(anyhow!("Limit must be positive").into());
    }
    
    if limit > 10000 {
        return Err(anyhow!("Limit cannot exceed 10000").into());
    }
    
    Ok(())
}

