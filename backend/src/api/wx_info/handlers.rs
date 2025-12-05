use axum::Json;
use std::collections::HashMap;

use crate::core::wx_info::get_wx_info;
use crate::utils::Result;
use super::models::*;

pub async fn get_wx_info_handler(
    wx_offs: axum::extract::State<HashMap<String, Vec<u32>>>,
) -> Result<Json<Vec<WxInfoResponse>>> {
    let infos = get_wx_info(&wx_offs)?;

    let responses: Vec<WxInfoResponse> = infos.into_iter().map(|info| {
        WxInfoResponse {
            pid: info.pid,
            version: info.version,
            account: info.account,
            mobile: info.mobile,
            nickname: info.nickname,
            mail: info.mail,
            wxid: info.wxid,
            key: info.key,
            wx_dir: info.wx_dir,
        }
    }).collect();

    Ok(Json(responses))
}

