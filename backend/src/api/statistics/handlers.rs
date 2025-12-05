use axum::{Json, extract::Path};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::db::msg::MsgHandler;
use crate::utils::{AppError, Result};
use super::models::*;

pub async fn get_contact_stat(
    Path(wxid): Path<String>,
    Json(req): Json<ContactStatRequest>
) -> Result<Json<ContactStatResponse>> {
    let db_path = PathBuf::from(&req.merge_path);
    if !db_path.exists() {
        return Err(AppError::NotFound(format!("Database not found: {}", req.merge_path)).into());
    }

    let handler = MsgHandler::new(db_path.to_str().unwrap())?;
    handler.add_indexes()?;

    let counts = handler.get_msg_count(Some(&wxid))?;
    let total = counts.get(&wxid).copied().unwrap_or(0);

    // 获取日期统计
    let date_stats = handler.get_date_count(Some(&wxid), req.start_time, req.end_time)?;

    // 计算发送和接收数量
    let mut sender_count = 0;
    let mut receiver_count = 0;
    for (_, stat) in &date_stats {
        if let Some(s) = stat.get("sender_count").and_then(|v| v.as_i64()) {
            sender_count += s;
        }
        if let Some(r) = stat.get("receiver_count").and_then(|v| v.as_i64()) {
            receiver_count += r;
        }
    }

    Ok(Json(ContactStatResponse {
        wxid,
        total_count: total,
        sender_count,
        receiver_count,
        date_stats,
    }))
}

pub async fn get_date_chat_stat(
    Json(req): Json<DateChatStatRequest>
) -> Result<Json<DateChatStatResponse>> {
    let db_path = PathBuf::from(&req.merge_path);
    if !db_path.exists() {
        return Err(AppError::NotFound(format!("Database not found: {}", req.merge_path)).into());
    }

    let handler = MsgHandler::new(db_path.to_str().unwrap())?;
    handler.add_indexes()?;

    let stats = handler.get_date_count(req.wxid.as_deref(), req.start_time, req.end_time)?;

    Ok(Json(DateChatStatResponse { stats }))
}

pub async fn get_date_heatmap(
    Json(req): Json<DateHeatmapRequest>
) -> Result<Json<DateHeatmapResponse>> {
    let db_path = PathBuf::from(&req.merge_path);
    if !db_path.exists() {
        return Err(AppError::NotFound(format!("Database not found: {}", req.merge_path)).into());
    }

    let handler = MsgHandler::new(db_path.to_str().unwrap())?;
    handler.add_indexes()?;

    let stats = handler.get_date_count(req.wxid.as_deref(), req.start_time, req.end_time)?;

    let data: Vec<HeatmapData> = stats
        .into_iter()
        .map(|(date, stat)| {
            let count = stat
                .get("total_count")
                .and_then(|v| v.as_i64())
                .unwrap_or(0);
            HeatmapData { date, count }
        })
        .collect();

    Ok(Json(DateHeatmapResponse { data }))
}

pub async fn get_top_talkers(
    Json(req): Json<TopTalkersRequest>
) -> Result<Json<TopTalkersResponse>> {
    let db_path = PathBuf::from(&req.merge_path);
    if !db_path.exists() {
        return Err(AppError::NotFound(format!("Database not found: {}", req.merge_path)).into());
    }

    let handler = MsgHandler::new(db_path.to_str().unwrap())?;
    handler.add_indexes()?;

    let top = req.top.unwrap_or(10);
    let talkers = handler.get_top_talkers(top, req.start_time, req.end_time)?;

    Ok(Json(TopTalkersResponse { talkers }))
}

