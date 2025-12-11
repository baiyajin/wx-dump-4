use crate::utils::Result;
use anyhow::Context;
use std::collections::HashMap;

/// Protobuf解析器
/// 用于解析微信消息中的BytesExtra字段（protobuf格式）
pub struct ProtobufParser;

impl ProtobufParser {
    /// 解析BytesExtra字段（protobuf格式）
    /// 微信的BytesExtra通常包含文件路径、URL等信息
    pub fn parse_bytes_extra(bytes: &[u8]) -> Result<HashMap<String, String>> {
        let mut result = HashMap::new();

        if bytes.is_empty() {
            return Ok(result);
        }

        // 尝试解析protobuf格式
        // 微信的BytesExtra通常是一个repeated字段的protobuf消息
        // 由于没有公开的.proto文件，我们使用启发式方法解析

        // 方法1: 尝试解析常见的protobuf字段
        if let Ok(parsed) = Self::parse_protobuf_fields(bytes) {
            return Ok(parsed);
        }

        // 方法2: 如果protobuf解析失败，回退到字符串搜索
        // 这可以处理一些简单的protobuf消息
        if let Ok(parsed) = Self::parse_fallback(bytes) {
            return Ok(parsed);
        }

        Ok(result)
    }

    /// 解析protobuf字段
    /// 使用protobuf的wire format解析
    fn parse_protobuf_fields(bytes: &[u8]) -> Result<HashMap<String, String>> {
        let mut result = HashMap::new();
        let mut offset = 0;

        while offset < bytes.len() {
            // 读取field number和wire type
            if offset >= bytes.len() {
                break;
            }

            let tag = bytes[offset];
            offset += 1;

            let field_number = tag >> 3;
            let wire_type = tag & 0x07;

            match wire_type {
                0 => {
                    // Varint
                    let (value, consumed) = Self::read_varint(&bytes[offset..])?;
                    offset += consumed;
                    // 这里可以根据field_number存储值
                }
                1 => {
                    // 64-bit
                    if offset + 8 > bytes.len() {
                        break;
                    }
                    offset += 8;
                }
                2 => {
                    // Length-delimited (string, bytes, embedded message)
                    let (length, consumed) = Self::read_varint(&bytes[offset..])?;
                    offset += consumed;

                    if offset + length as usize > bytes.len() {
                        break;
                    }

                    let data = &bytes[offset..offset + length as usize];
                    offset += length as usize;

                    // 尝试解析为字符串
                    if let Ok(text) = String::from_utf8(data.to_vec()) {
                        // 检查是否是文件路径
                        if text.contains("FileStorage") {
                            result.insert("file_path".to_string(), text);
                        } else if text.starts_with("http://") || text.starts_with("https://") {
                            result.insert("url".to_string(), text);
                        } else if field_number == 2 {
                            // 常见的文件路径字段
                            result.insert("path".to_string(), text);
                        }
                    }
                }
                5 => {
                    // 32-bit
                    if offset + 4 > bytes.len() {
                        break;
                    }
                    offset += 4;
                }
                _ => {
                    // 未知类型，跳过
                    break;
                }
            }
        }

        Ok(result)
    }

    /// 读取varint值
    fn read_varint(bytes: &[u8]) -> Result<(u64, usize)> {
        let mut value = 0u64;
        let mut shift = 0;
        let mut consumed = 0;

        for &byte in bytes.iter().take(10) {
            consumed += 1;
            value |= ((byte & 0x7F) as u64) << shift;

            if (byte & 0x80) == 0 {
                break;
            }

            shift += 7;
            if shift >= 64 {
                return Err(anyhow::anyhow!("Varint too long").into());
            }
        }

        Ok((value, consumed))
    }

    /// 回退解析方法（当protobuf解析失败时使用）
    fn parse_fallback(bytes: &[u8]) -> Result<HashMap<String, String>> {
        let mut result = HashMap::new();

        // 尝试将bytes转换为字符串
        let text = String::from_utf8_lossy(bytes);

        // 使用正则表达式提取路径和URL
        use regex::Regex;

        // 提取FileStorage路径
        if let Ok(re) = Regex::new(r"FileStorage[^\x00]*") {
            for cap in re.find_iter(&text) {
                let path = cap.as_str().trim_matches('\x00').trim();
                if !path.is_empty() {
                    result.insert("file_path".to_string(), path.to_string());
                    break;
                }
            }
        }

        // 提取URL
        if let Ok(re) = Regex::new(r"https?://[^\x00\s]+") {
            for cap in re.find_iter(&text) {
                let url = cap.as_str().trim_matches('\x00').trim();
                if !url.is_empty() {
                    result.insert("url".to_string(), url.to_string());
                    break;
                }
            }
        }

        Ok(result)
    }

    /// 从protobuf解析中提取图片路径
    pub fn extract_image_path(bytes: &[u8]) -> Option<String> {
        if let Ok(parsed) = Self::parse_bytes_extra(bytes) {
            // 优先使用file_path
            if let Some(path) = parsed.get("file_path") {
                if path.contains("Image") {
                    return Some(path.clone());
                }
            }
            // 其次使用path
            if let Some(path) = parsed.get("path") {
                if path.contains("Image") {
                    return Some(path.clone());
                }
            }
        }
        None
    }

    /// 从protobuf解析中提取视频路径
    pub fn extract_video_path(bytes: &[u8]) -> Option<String> {
        if let Ok(parsed) = Self::parse_bytes_extra(bytes) {
            if let Some(path) = parsed.get("file_path") {
                if path.contains("mp4") || path.contains("Video") {
                    return Some(path.clone());
                }
            }
            if let Some(path) = parsed.get("path") {
                if path.contains("mp4") || path.contains("Video") {
                    return Some(path.clone());
                }
            }
        }
        None
    }

    /// 从protobuf解析中提取文件URL
    pub fn extract_file_url(bytes: &[u8]) -> Option<String> {
        if let Ok(parsed) = Self::parse_bytes_extra(bytes) {
            parsed.get("url").cloned()
        } else {
            None
        }
    }
}
