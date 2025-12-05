use crate::utils::Result;
use anyhow::Context;
use aes::Aes256;
use cbc::cipher::{BlockDecryptMut, KeyIvInit};
use hmac::{Hmac, Mac};
use pbkdf2::pbkdf2_hmac;
use sha1::Sha1;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

const SQLITE_FILE_HEADER: &[u8] = b"SQLite format 3\x00";
const KEY_SIZE: usize = 32;
const PAGE_SIZE: usize = 4096;

pub fn decrypt_db(key: &str, db_path: &Path, out_path: &Path) -> Result<()> {
    // 验证输入
    if !db_path.exists() || !db_path.is_file() {
        return Err(anyhow::anyhow!("Database file not found: {:?}", db_path).into());
    }

    if let Some(parent) = out_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create output directory: {:?}", parent))?;
        }
    }

    if key.len() != 64 {
        return Err(anyhow::anyhow!("Key length must be 64 hex characters").into());
    }

    // 读取加密数据库
    let encrypted_data = fs::read(db_path)
        .with_context(|| format!("Failed to read database: {:?}", db_path))?;

    if encrypted_data.len() < 16 {
        return Err(anyhow::anyhow!("Database file too small").into());
    }

    // 提取salt和第一页
    let salt = &encrypted_data[0..16];
    let first_page = &encrypted_data[16..4096];

    if salt.len() != 16 {
        return Err(anyhow::anyhow!("Invalid salt size").into());
    }

    // 解析密钥
    let password = hex::decode(key.trim())
        .with_context(|| "Failed to decode hex key")?;

    // 计算mac_salt
    let mac_salt: Vec<u8> = salt.iter().map(|&b| b ^ 58).collect();

    // PBKDF2 派生密钥
    let mut byte_hmac = [0u8; KEY_SIZE];
    pbkdf2_hmac::<Sha1>(&password, salt, 64000, &mut byte_hmac);

    // 计算mac_key
    let mut mac_key = [0u8; KEY_SIZE];
    pbkdf2_hmac::<Sha1>(&byte_hmac, &mac_salt, 2, &mut mac_key);

    // 验证MAC
    let mut mac = Hmac::<Sha1>::new_from_slice(&mac_key)
        .map_err(|e| anyhow::anyhow!("Failed to create HMAC: {}", e))?;
    mac.update(&encrypted_data[16..4064]);
    mac.update(b"\x01\x00\x00\x00");
    let computed_mac = mac.finalize().into_bytes();

    let stored_mac = &first_page[4064 - 32 - 12..4064 - 12];
    if computed_mac.as_slice() != stored_mac {
        return Err(anyhow::anyhow!("Key verification failed - incorrect key").into());
    }

    // 解密数据库
    let mut output = File::create(out_path)
        .with_context(|| format!("Failed to create output file: {:?}", out_path))?;

    // 写入SQLite文件头
    output.write_all(SQLITE_FILE_HEADER)
        .with_context(|| "Failed to write SQLite header")?;

    // 解密每一页
    let total_pages = (encrypted_data.len() + PAGE_SIZE - 1) / PAGE_SIZE;
    
    for page_num in 0..total_pages {
        let page_start = if page_num == 0 { 16 } else { page_num * PAGE_SIZE };
        let page_end = ((page_num + 1) * PAGE_SIZE).min(encrypted_data.len());
        
        if page_start >= encrypted_data.len() {
            break;
        }

        let page_data = &encrypted_data[page_start..page_end];
        
        if page_data.len() < 48 {
            // 最后一页可能不足4096字节
            output.write_all(page_data)?;
            break;
        }

        // 提取IV和加密数据
        let iv = &page_data[page_data.len() - 48..page_data.len() - 32];
        let encrypted_page = &page_data[..page_data.len() - 48];
        let footer = &page_data[page_data.len() - 48..];

        // 解密 - 简化实现，使用AES-256-CBC
        type Aes256CbcDec = cbc::Decryptor<Aes256>;
        let cipher = Aes256CbcDec::new_from_slices(&byte_hmac, iv)
            .map_err(|e| anyhow::anyhow!("Failed to create decryptor: {}", e))?;

        // 解密数据（需要对齐到16字节）
        let mut decrypted = encrypted_page.to_vec();
        let padding = (16 - (decrypted.len() % 16)) % 16;
        decrypted.extend(vec![0u8; padding]);

        cipher.decrypt_padded_mut::<cbc::cipher::block_padding::Pkcs7>(&mut decrypted)
            .map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))?;

        // 移除填充
        if decrypted.len() > encrypted_page.len() {
            decrypted.truncate(encrypted_page.len());
        }

        output.write_all(&decrypted)?;
        output.write_all(footer)?;
    }

    Ok(())
}

pub fn batch_decrypt(
    key: &str,
    db_paths: &[PathBuf],
    out_dir: &Path,
) -> Result<Vec<(PathBuf, PathBuf)>> {
    if !out_dir.exists() {
        fs::create_dir_all(out_dir)
            .with_context(|| format!("Failed to create output directory: {:?}", out_dir))?;
    }

    let mut results = Vec::new();

    for db_path in db_paths {
        let file_name = db_path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| anyhow::anyhow!("Invalid file name"))?;

        let out_path = out_dir.join(format!("de_{}", file_name));

        match decrypt_db(key, db_path, &out_path) {
            Ok(_) => {
                results.push((db_path.clone(), out_path));
            }
            Err(e) => {
                tracing::error!("Failed to decrypt {:?}: {}", db_path, e);
            }
        }
    }

    Ok(results)
}

