use axum::{Json, Router, routing::{get, post}, extract::Path};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

use crate::core::wx_info::{get_wx_info, get_info_details};
use crate::models::wx::WxInfoResponse;
use crate::config::{load_wx_offs, save_wx_offs};
use crate::utils::{AppError, Result};

pub fn router(wx_offs: HashMap<String, Vec<u32>>) -> Router {
    let wx_offs = Arc::new(wx_offs);
    
    Router::new()
        .route(
            "/api/wx/info",
            post({
                let wx_offs = wx_offs.clone();
                move |_| get_wx_info_handler(wx_offs)
            }),
        )
        .route(
            "/api/wx/info/:pid",
            get({
                let wx_offs = wx_offs.clone();
                move |path: Path<u32>| get_wx_info_by_pid_handler(path, wx_offs)
            }),
        )
        .route(
            "/api/wx/version/list",
            get(get_version_list_handler),
        )
        .route(
            "/api/wx/version/offs",
            post(add_version_offs_handler),
        )
}

async fn get_wx_info_handler(
    wx_offs: Arc<HashMap<String, Vec<u32>>>,
) -> Result<Json<Vec<WxInfoResponse>>> {
    let infos = get_wx_info(&wx_offs)?;
    
    let responses: Vec<WxInfoResponse> = infos
        .into_iter()
        .map(|info| WxInfoResponse {
            pid: info.pid,
            version: info.version,
            account: info.account,
            mobile: info.mobile,
            nickname: info.nickname,
            mail: info.mail,
            wxid: info.wxid,
            key: info.key,
            wx_dir: info.wx_dir,
        })
        .collect();

    Ok(Json(responses))
}

async fn get_wx_info_by_pid_handler(
    Path(pid): Path<u32>,
    wx_offs: Arc<HashMap<String, Vec<u32>>>,
) -> Result<Json<WxInfoResponse>> {
    let info = get_info_details(pid, &wx_offs)
        .map_err(|_| AppError::NotFound(format!("WeChat process {} not found", pid)))?;
    
    Ok(Json(WxInfoResponse {
        pid: info.pid,
        version: info.version,
        account: info.account,
        mobile: info.mobile,
        nickname: info.nickname,
        mail: info.mail,
        wxid: info.wxid,
        key: info.key,
        wx_dir: info.wx_dir,
    }))
}

async fn get_version_list_handler() -> Result<Json<VersionListResponse>> {
    let wx_offs = load_wx_offs()?;
    let versions: Vec<String> = wx_offs.keys().cloned().collect();
    
    Ok(Json(VersionListResponse { versions }))
}

#[derive(Debug, Deserialize)]
pub struct AddVersionOffsRequest {
    pub version: String,
    pub offs: Vec<u32>,
}

#[derive(Debug, Serialize)]
pub struct VersionListResponse {
    pub versions: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct AddVersionOffsResponse {
    pub success: bool,
    pub message: String,
}

async fn add_version_offs_handler(
    Json(req): Json<AddVersionOffsRequest>,
) -> Result<Json<AddVersionOffsResponse>> {
    let mut wx_offs = load_wx_offs()?;
    
    if wx_offs.contains_key(&req.version) {
        return Ok(Json(AddVersionOffsResponse {
            success: false,
            message: format!("版本 {} 已存在", req.version),
        }));
    }
    
    wx_offs.insert(req.version.clone(), req.offs);
    save_wx_offs(&wx_offs)?;
    
    Ok(Json(AddVersionOffsResponse {
        success: true,
        message: format!("成功添加版本 {} 的偏移量", req.version),
    }))
}

