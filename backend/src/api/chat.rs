use axum::{Json, Router, routing::{get, post}, extract::Path};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use crate::db::msg::MsgHandler;
use crate::db::contact::ContactHandler;
use crate::utils::{AppError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatContactsRequest {
    pub merge_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatContactsResponse {
    pub contacts: Vec<ContactInfo>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactInfo {
    pub wxid: String,
    pub msg_count: i64,
    pub sender_count: i64,
    pub receiver_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MsgCountRequest {
    pub merge_path: String,
    pub wxid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MsgCountResponse {
    pub counts: HashMap<String, i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MsgListRequest {
    pub merge_path: String,
    pub wxid: String,
    pub start: i64,
    pub limit: i64,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MsgListResponse {
    pub messages: Vec<MessageResponse>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageResponse {
    pub id: i64,
    pub local_id: i64,
    pub msg_svr_id: i64,
    pub msg_type: i32,
    pub sub_type: i32,
    pub type_name: String,
    pub create_time: i64,
    pub create_time_str: String,
    pub is_sender: i32,
    pub talker: String,
    pub str_talker: String,
    pub content: String,
    pub display_content: String,
    pub src: String,
    pub extra: serde_json::Value,
}

pub fn router() -> Router {
    Router::new()
        .route("/api/chat/contacts", post(get_contacts))
        .route("/api/chat/contacts/:wxid", get(get_contact_detail))
        .route("/api/chat/msg/count", post(get_msg_count))
        .route("/api/chat/msg/list", post(get_msg_list))
}

async fn get_contacts(Json(req): Json<ChatContactsRequest>) -> Result<Json<ChatContactsResponse>> {
    let db_path = PathBuf::from(&req.merge_path);
    if !db_path.exists() {
        return Err(AppError::NotFound(format!("Database not found: {}", req.merge_path)).into());
    }

    let handler = MsgHandler::new(db_path.to_str().unwrap())?;
    handler.add_indexes()?;

    let counts = handler.get_msg_count(None)?;
    let total = counts.get("total").copied().unwrap_or(0);

    let mut contacts = Vec::new();
    for (wxid, count) in counts.iter() {
        if wxid != "total" {
            // 这里需要从数据库获取更详细的信息
            contacts.push(ContactInfo {
                wxid: wxid.clone(),
                msg_count: *count,
                sender_count: 0, // TODO: 从数据库获取
                receiver_count: 0, // TODO: 从数据库获取
            });
        }
    }

    // 按消息数量排序
    contacts.sort_by(|a, b| b.msg_count.cmp(&a.msg_count));

    Ok(Json(ChatContactsResponse { contacts, total }))
}

async fn get_msg_count(Json(req): Json<MsgCountRequest>) -> Result<Json<MsgCountResponse>> {
    let db_path = PathBuf::from(&req.merge_path);
    if !db_path.exists() {
        return Err(AppError::NotFound(format!("Database not found: {}", req.merge_path)).into());
    }

    let handler = MsgHandler::new(db_path.to_str().unwrap())?;
    handler.add_indexes()?;

    let counts = handler.get_msg_count(req.wxid.as_deref())?;

    Ok(Json(MsgCountResponse { counts }))
}

async fn get_msg_list(Json(req): Json<MsgListRequest>) -> Result<Json<MsgListResponse>> {
    let db_path = PathBuf::from(&req.merge_path);
    if !db_path.exists() {
        return Err(AppError::NotFound(format!("Database not found: {}", req.merge_path)).into());
    }

    let handler = MsgHandler::new(db_path.to_str().unwrap())?;
    handler.add_indexes()?;

    let messages = handler.get_msg_list(
        Some(&req.wxid),
        req.start,
        req.limit,
        req.start_time,
        req.end_time,
    )?;

    let total = handler.get_msg_count(Some(&req.wxid))?
        .get(&req.wxid)
        .copied()
        .unwrap_or(0);

    let msg_responses: Vec<MessageResponse> = messages.into_iter().map(|msg| {
        MessageResponse {
            id: msg.id,
            local_id: msg.local_id,
            msg_svr_id: msg.msg_svr_id,
            msg_type: msg.msg_type,
            sub_type: msg.sub_type,
            type_name: msg.type_name,
            create_time: msg.create_time,
            create_time_str: msg.create_time_str,
            is_sender: msg.is_sender,
            talker: msg.talker,
            str_talker: msg.str_talker,
            content: msg.content,
            display_content: msg.display_content,
            src: msg.src,
            extra: msg.extra,
        }
    }).collect();

    Ok(Json(MsgListResponse {
        messages: msg_responses,
        total,
    }))
}

async fn get_contact_detail(Path(wxid): Path<String>, Json(req): Json<ChatContactsRequest>) -> Result<Json<serde_json::Value>> {
    let db_path = PathBuf::from(&req.merge_path);
    if !db_path.exists() {
        return Err(AppError::NotFound(format!("Database not found: {}", req.merge_path)).into());
    }

    // 尝试从MicroMsg.db获取联系人信息
    let micro_db_path = db_path.parent()
        .and_then(|p| p.parent())
        .map(|p| p.join("MicroMsg.db"))
        .ok_or_else(|| AppError::NotFound("Cannot find MicroMsg.db path".to_string()))?;

    if micro_db_path.exists() {
        let handler = ContactHandler::new(micro_db_path.to_str().unwrap())?;
        if let Some(contact) = handler.get_contact(&wxid)? {
            return Ok(Json(serde_json::json!(contact)));
        }
    }

    // 如果找不到，返回基本信息
    Ok(Json(serde_json::json!({
        "wxid": wxid,
        "nickname": None::<String>,
        "remark": None::<String>,
    })))
}

