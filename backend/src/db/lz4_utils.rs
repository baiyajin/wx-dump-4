use crate::utils::Result;
use anyhow::Context;

/// LZ4解压工具
pub struct Lz4Utils;

impl Lz4Utils {
    /// 解压CompressContent
    pub fn decompress(compressed: &[u8]) -> Result<String> {
        if compressed.is_empty() {
            return Ok(String::new());
        }

        // 估算解压后的大小（通常是压缩数据的8倍，但不超过1MB）
        let estimated_size = (compressed.len() * 8).min(1024 * 1024);
        
        // 使用lz4_flex库解压
        let decompressed = lz4_flex::decompress_size_prepended(compressed)
            .or_else(|_| {
                // 如果失败，尝试使用固定大小解压
                let mut output = vec![0u8; estimated_size];
                match lz4_flex::decompress_into(compressed, &mut output) {
                    Ok(size) => {
                        output.truncate(size);
                        Ok(output)
                    }
                    Err(e) => Err(e),
                }
            })
            .context("Failed to decompress LZ4 data")?;

        // 移除null字节
        let cleaned: Vec<u8> = decompressed.into_iter().filter(|&b| b != 0).collect();

        // 转换为UTF-8字符串
        String::from_utf8(cleaned)
            .context("Failed to convert decompressed data to UTF-8")
            .map_err(Into::into)
    }

    /// 尝试解压，失败返回空字符串
    pub fn decompress_or_empty(compressed: &[u8]) -> String {
        Self::decompress(compressed).unwrap_or_default()
    }
}

