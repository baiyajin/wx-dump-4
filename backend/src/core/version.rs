use std::collections::HashMap;

pub fn detect_address_len(version: &str) -> usize {
    let parts: Vec<u32> = version
        .split('.')
        .map(|s| s.parse().unwrap_or(0))
        .collect();
    
    if parts.is_empty() {
        return 8; // 默认64位
    }
    
    // 4.0+ 版本使用64位地址
    if parts[0] >= 4 {
        return 8;
    }
    
    // 3.10+ 版本使用64位地址
    if parts[0] == 3 && parts.len() > 1 && parts[1] >= 10 {
        return 8;
    }
    
    // 3.9.2+ 某些版本使用64位
    if parts[0] == 3 && parts.len() > 2 {
        if parts[1] == 9 && parts[2] >= 2 {
            return 8;
        }
    }
    
    // 默认32位（旧版本）
    4
}

pub fn get_version_offset(
    version: &str,
    wx_offs: &HashMap<String, Vec<u32>>,
) -> Option<&Vec<u32>> {
    wx_offs.get(version)
}

pub fn parse_version(version_str: &str) -> Option<(u32, u32, u32, u32)> {
    let parts: Vec<u32> = version_str
        .split('.')
        .map(|s| s.parse().ok())
        .collect::<Option<Vec<_>>>()?;
    
    match parts.len() {
        4 => Some((parts[0], parts[1], parts[2], parts[3])),
        3 => Some((parts[0], parts[1], parts[2], 0)),
        _ => None,
    }
}

