use axum::{Json, Router, routing::post};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;

use crate::core::wx_info::get_wx_info;
use crate::models::wx::WxInfoResponse;
use crate::utils::{AppError, Result};

pub fn router(wx_offs: HashMap<String, Vec<u32>>) -> Router {
    let wx_offs = Arc::new(wx_offs);
    
    Router::new().route(
        "/api/wx/info",
        post({
            let wx_offs = wx_offs.clone();
            move |_| get_wx_info_handler(wx_offs)
        }),
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

