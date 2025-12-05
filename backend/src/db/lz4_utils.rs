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

        // 估算解压后的大小（通常是压缩数据的8倍）
        let estimated_size = compressed.len() * 8;
        
        let mut decompressed = Vec::with_capacity(estimated_size);
        
        // 使用lz4库解压
        let mut decoder = lz4::Decoder::new(compressed)
            .context("Failed to create LZ4 decoder")?;
        
        std::io::copy(&mut decoder, &mut decompressed)
            .context("Failed to decompress LZ4 data")?;

        // 移除null字节
        decompressed.retain(|&b| b != 0);

        // 转换为UTF-8字符串
        String::from_utf8(decompressed)
            .context("Failed to convert decompressed data to UTF-8")
            .map_err(Into::into)
    }

    /// 尝试解压，失败返回空字符串
    pub fn decompress_or_empty(compressed: &[u8]) -> String {
        Self::decompress(compressed).unwrap_or_default()
    }
}

