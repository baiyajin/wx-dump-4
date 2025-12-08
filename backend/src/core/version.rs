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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_address_len() {
        // 测试4.0+版本（64位）
        assert_eq!(detect_address_len("4.0.0.0"), 8);
        assert_eq!(detect_address_len("4.1.0.0"), 8);
        assert_eq!(detect_address_len("5.0.0.0"), 8);
        
        // 测试3.10+版本（64位）
        assert_eq!(detect_address_len("3.10.0.0"), 8);
        assert_eq!(detect_address_len("3.11.0.0"), 8);
        
        // 测试3.9.2+版本（64位）
        assert_eq!(detect_address_len("3.9.2.0"), 8);
        assert_eq!(detect_address_len("3.9.3.0"), 8);
        
        // 测试旧版本（32位）
        assert_eq!(detect_address_len("3.9.1.0"), 4);
        assert_eq!(detect_address_len("3.8.0.0"), 4);
        
        // 测试空版本（默认64位）
        assert_eq!(detect_address_len(""), 8);
    }

    #[test]
    fn test_parse_version() {
        // 测试4段版本号
        assert_eq!(parse_version("4.0.0.0"), Some((4, 0, 0, 0)));
        assert_eq!(parse_version("3.9.2.1"), Some((3, 9, 2, 1)));
        
        // 测试3段版本号
        assert_eq!(parse_version("4.0.0"), Some((4, 0, 0, 0)));
        assert_eq!(parse_version("3.9.2"), Some((3, 9, 2, 0)));
        
        // 测试无效版本号
        assert_eq!(parse_version("invalid"), None);
        assert_eq!(parse_version("4"), None);
        assert_eq!(parse_version(""), None);
    }

    #[test]
    fn test_get_version_offset() {
        let mut wx_offs = HashMap::new();
        wx_offs.insert("4.0.0.0".to_string(), vec![0x1234, 0x5678]);
        wx_offs.insert("3.9.2.0".to_string(), vec![0xABCD, 0xEF00]);
        
        // 测试存在的版本
        assert!(get_version_offset("4.0.0.0", &wx_offs).is_some());
        assert_eq!(
            get_version_offset("4.0.0.0", &wx_offs).unwrap(),
            &vec![0x1234, 0x5678]
        );
        
        // 测试不存在的版本
        assert!(get_version_offset("5.0.0.0", &wx_offs).is_none());
    }
}

