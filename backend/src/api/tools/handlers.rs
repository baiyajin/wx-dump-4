use axum::Json;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use crate::config::{load_wx_offs, save_wx_offs};
use crate::core::decryption::decrypt_db;
use crate::core::wx_info::get_wx_info;
use crate::db::merge::merge_databases;
use crate::utils::{AppError, Result};
use super::models::*;

pub async fn get_bias(
    Json(req): Json<GetBiasRequest>,
) -> Result<Json<GetBiasResponse>> {
    let wx_offs = load_wx_offs()?;
    
    if let Some(bias) = wx_offs.get(&req.version) {
        Ok(Json(GetBiasResponse {
            success: true,
            message: format!("成功获取版本 {} 的偏移量", req.version),
            bias: Some(bias.clone()),
        }))
    } else {
        Ok(Json(GetBiasResponse {
            success: false,
            message: format!("版本 {} 的偏移量不存在", req.version),
            bias: None,
        }))
    }
}

pub async fn decrypt_db(
    Json(req): Json<DecryptDbRequest>,
) -> Result<Json<DecryptDbResponse>> {
    let db_path = PathBuf::from(&req.db_path);
    let out_path = PathBuf::from(&req.out_path);
    
    if !db_path.exists() {
        return Ok(Json(DecryptDbResponse {
            success: false,
            message: format!("数据库文件不存在: {}", req.db_path),
            out_path: None,
        }));
    }
    
    match decrypt_db(&req.key, &db_path, &out_path) {
        Ok(_) => Ok(Json(DecryptDbResponse {
            success: true,
            message: "解密成功".to_string(),
            out_path: Some(req.out_path),
        })),
        Err(e) => Ok(Json(DecryptDbResponse {
            success: false,
            message: format!("解密失败: {}", e),
            out_path: None,
        })),
    }
}

pub async fn merge_db(
    Json(req): Json<MergeDbRequest>,
) -> Result<Json<MergeDbResponse>> {
    if req.db_paths.is_empty() {
        return Ok(Json(MergeDbResponse {
            success: false,
            message: "数据库路径列表为空".to_string(),
            out_path: None,
        }));
    }
    
    // 检查所有数据库文件是否存在
    for db_path in &req.db_paths {
        let path = PathBuf::from(db_path);
        if !path.exists() {
            return Ok(Json(MergeDbResponse {
                success: false,
                message: format!("数据库文件不存在: {}", db_path),
                out_path: None,
            }));
        }
    }
    
    let db_paths: Vec<PathBuf> = req.db_paths.iter().map(PathBuf::from).collect();
    let out_path = PathBuf::from(&req.out_path);
    
    match merge_databases(&db_paths, &out_path) {
        Ok(_) => Ok(Json(MergeDbResponse {
            success: true,
            message: format!("成功合并 {} 个数据库", db_paths.len()),
            out_path: Some(req.out_path),
        })),
        Err(e) => Ok(Json(MergeDbResponse {
            success: false,
            message: format!("合并失败: {}", e),
            out_path: None,
        })),
    }
}

pub async fn get_wxinfo() -> Result<Json<WxInfoToolResponse>> {
    let wx_offs = load_wx_offs()?;
    
    match get_wx_info(&wx_offs) {
        Ok(infos) => {
            let info_json = serde_json::to_value(infos)
                .map_err(|e| AppError::InternalError(format!("序列化失败: {}", e)))?;
            
            Ok(Json(WxInfoToolResponse {
                success: true,
                message: "成功获取微信信息".to_string(),
                info: Some(info_json),
            }))
        }
        Err(e) => Ok(Json(WxInfoToolResponse {
            success: false,
            message: format!("获取微信信息失败: {}", e),
            info: None,
        })),
    }
}

