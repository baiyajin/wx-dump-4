use crate::db::bytes_extra::BytesExtraParser;
use crate::db::lz4_utils::Lz4Utils;
use crate::utils::Result;
use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;

/// 解析消息内容
pub struct MessageParser;

impl MessageParser {
    /// 解析图片消息的路径
    pub fn parse_image_path(bytes_extra: &[u8]) -> Option<String> {
        BytesExtraParser::extract_image_path(bytes_extra)
    }

    /// 解析语音消息
    pub fn parse_voice_message(content: &str) -> HashMap<String, String> {
        let mut result = HashMap::new();
        
        // 简单的XML解析（后续需要更完善的解析）
        if let Some(voicelength) = Self::extract_xml_value(content, "voicelength") {
            if let Ok(length) = voicelength.parse::<f64>() {
                let seconds = length / 1000.0;
                result.insert("voicelength".to_string(), format!("{:.2}", seconds));
            }
        }
        
        if let Some(transtext) = Self::extract_xml_value(content, "transtext") {
            result.insert("transtext".to_string(), transtext);
        }
        
        result
    }

    /// 解析视频消息
    pub fn parse_video_message(bytes_extra: &[u8]) -> Option<String> {
        BytesExtraParser::extract_video_path(bytes_extra)
    }

    /// 解析文件消息
    pub fn parse_file_message(bytes_extra: &[u8]) -> Option<String> {
        BytesExtraParser::extract_file_url(bytes_extra)
            .or_else(|| BytesExtraParser::extract_file_storage_path(bytes_extra))
    }

    /// 解析分享消息（49类型）
    pub fn parse_share_message(compress_content: &[u8], content: &str) -> HashMap<String, Value> {
        let mut result = HashMap::new();
        
        // 先尝试解压CompressContent
        let decompressed = if !compress_content.is_empty() {
            Lz4Utils::decompress_or_empty(compress_content)
        } else {
            String::new()
        };
        
        // 优先使用解压后的内容
        let xml_content = if !decompressed.is_empty() {
            &decompressed
        } else {
            content
        };
        
        // 尝试从XML内容解析
        if let Some(title) = Self::extract_xml_value(xml_content, "title") {
            result.insert("title".to_string(), Value::String(title));
        }
        
        if let Some(des) = Self::extract_xml_value(xml_content, "des") {
            result.insert("des".to_string(), Value::String(des));
        }
        
        if let Some(url) = Self::extract_xml_value(xml_content, "url") {
            result.insert("url".to_string(), Value::String(url));
        }
        
        result
    }

    /// 从XML字符串中提取值
    fn extract_xml_value(xml: &str, tag: &str) -> Option<String> {
        let pattern = format!(r"<{}><!\[CDATA\[(.*?)\]\]></{}>", tag, tag);
        let re = Regex::new(&pattern).ok()?;
        re.captures(xml).and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()))
    }

    /// 解析位置消息
    pub fn parse_location_message(content: &str) -> HashMap<String, String> {
        let mut result = HashMap::new();
        
        if let Some(x) = Self::extract_xml_value(content, "x") {
            result.insert("latitude".to_string(), x);
        }
        
        if let Some(y) = Self::extract_xml_value(content, "y") {
            result.insert("longitude".to_string(), y);
        }
        
        if let Some(label) = Self::extract_xml_value(content, "label") {
            result.insert("label".to_string(), label);
        }
        
        if let Some(poiname) = Self::extract_xml_value(content, "poiname") {
            result.insert("poiname".to_string(), poiname);
        }
        
        result
    }

    /// 解析表情消息
    pub fn parse_emoji_message(content: &str, bytes_extra: &[u8]) -> Option<String> {
        // 先尝试从content中获取cdnurl
        if let Some(cdnurl) = Self::extract_xml_value(content, "cdnurl") {
            return Some(cdnurl);
        }
        
        // 从BytesExtra中提取URL
        BytesExtraParser::extract_file_url(bytes_extra)
    }
}

