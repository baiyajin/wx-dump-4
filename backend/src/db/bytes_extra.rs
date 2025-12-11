use crate::utils::Result;
use crate::db::protobuf_parser::ProtobufParser;
use regex::Regex;
use std::collections::HashMap;

/// BytesExtra解析器
/// 优先使用Protobuf解析，失败时回退到正则表达式
pub struct BytesExtraParser;

impl BytesExtraParser {
    /// 从BytesExtra中提取FileStorage路径
    pub fn extract_file_storage_path(bytes_extra: &[u8]) -> Option<String> {
        if bytes_extra.is_empty() {
            return None;
        }

        // 尝试将bytes转换为字符串，查找FileStorage路径
        // 这是一个简化的实现，完整实现需要使用protobuf解析
        let text = String::from_utf8_lossy(bytes_extra);
        
        // 使用正则表达式匹配FileStorage路径
        let pattern = r"FileStorage[^']*";
        if let Ok(re) = Regex::new(pattern) {
            if let Some(captures) = re.find(&text) {
                let path = captures.as_str();
                // 清理路径
                let cleaned_path = path
                    .replace("\\", "/")
                    .replace("'", "")
                    .trim()
                    .to_string();
                if !cleaned_path.is_empty() {
                    return Some(cleaned_path);
                }
            }
        }

        None
    }

    /// 提取图片路径（优先Image目录）
    /// 优先使用Protobuf解析，失败时使用正则表达式
    pub fn extract_image_path(bytes_extra: &[u8]) -> Option<String> {
        if bytes_extra.is_empty() {
            return None;
        }

        // 优先尝试Protobuf解析
        if let Some(path) = ProtobufParser::extract_image_path(bytes_extra) {
            return Some(path);
        }

        // 回退到正则表达式解析
        let text = String::from_utf8_lossy(bytes_extra);
        
        // 匹配所有FileStorage路径
        let pattern = r"FileStorage[^']*";
        if let Ok(re) = Regex::new(pattern) {
            let mut paths: Vec<String> = re
                .find_iter(&text)
                .map(|m| {
                    m.as_str()
                        .replace("\\", "/")
                        .replace("'", "")
                        .trim()
                        .to_string()
                })
                .filter(|p| !p.is_empty())
                .collect();

            // 优先选择包含Image的路径
            paths.sort_by(|a, b| {
                let a_has_image = a.contains("Image");
                let b_has_image = b.contains("Image");
                b_has_image.cmp(&a_has_image)
            });

            return paths.first().cloned();
        }

        None
    }

    /// 提取视频路径（优先mp4）
    /// 优先使用Protobuf解析，失败时使用正则表达式
    pub fn extract_video_path(bytes_extra: &[u8]) -> Option<String> {
        if bytes_extra.is_empty() {
            return None;
        }

        // 优先尝试Protobuf解析
        if let Some(path) = ProtobufParser::extract_video_path(bytes_extra) {
            return Some(path);
        }

        // 回退到正则表达式解析
        let text = String::from_utf8_lossy(bytes_extra);
        
        let pattern = r"FileStorage[^']*";
        if let Ok(re) = Regex::new(pattern) {
            let mut paths: Vec<String> = re
                .find_iter(&text)
                .map(|m| {
                    m.as_str()
                        .replace("\\", "/")
                        .replace("'", "")
                        .trim()
                        .to_string()
                })
                .filter(|p| !p.is_empty())
                .collect();

            // 优先选择包含mp4的路径
            paths.sort_by(|a, b| {
                let a_has_mp4 = a.contains("mp4");
                let b_has_mp4 = b.contains("mp4");
                b_has_mp4.cmp(&a_has_mp4)
            });

            return paths.first().cloned();
        }

        None
    }

    /// 提取文件URL
    /// 优先使用Protobuf解析，失败时使用正则表达式
    pub fn extract_file_url(bytes_extra: &[u8]) -> Option<String> {
        if bytes_extra.is_empty() {
            return None;
        }

        // 优先尝试Protobuf解析
        if let Some(url) = ProtobufParser::extract_file_url(bytes_extra) {
            return Some(url);
        }

        // 回退到正则表达式解析
        let text = String::from_utf8_lossy(bytes_extra);
        
        // 尝试匹配URL
        let url_pattern = r"https?://[^\s'\"<>]+";
        if let Ok(re) = Regex::new(url_pattern) {
            if let Some(captures) = re.find(&text) {
                return Some(captures.as_str().to_string());
            }
        }

        None
    }
}

