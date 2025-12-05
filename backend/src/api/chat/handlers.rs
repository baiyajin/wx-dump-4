use axum::Json;
use axum::extract::Path;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::db::msg::MsgHandler;
use crate::db::contact::ContactHandler;
use crate::utils::{AppError, Result};
use super::models::*;

pub async fn get_contacts(Json(req): Json<ChatContactsRequest>) -> Result<Json<ChatContactsResponse>> {
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
            contacts.push(ContactInfo {
                wxid: wxid.clone(),
                msg_count: *count,
                sender_count: 0,
                receiver_count: 0,
            });
        }
    }

    contacts.sort_by(|a, b| b.msg_count.cmp(&a.msg_count));

    Ok(Json(ChatContactsResponse { contacts, total }))
}

pub async fn get_msg_count(Json(req): Json<MsgCountRequest>) -> Result<Json<MsgCountResponse>> {
    let db_path = PathBuf::from(&req.merge_path);
    if !db_path.exists() {
        return Err(AppError::NotFound(format!("Database not found: {}", req.merge_path)).into());
    }

    let handler = MsgHandler::new(db_path.to_str().unwrap())?;
    handler.add_indexes()?;

    let counts = handler.get_msg_count(req.wxid.as_deref())?;

    Ok(Json(MsgCountResponse { counts }))
}

pub async fn get_msg_list(Json(req): Json<MsgListRequest>) -> Result<Json<MsgListResponse>> {
    let db_path = PathBuf::from(&req.merge_path);
    if !db_path.exists() {
        return Err(AppError::NotFound(format!("Database not found: {}", req.merge_path)).into());
    }

    let handler = MsgHandler::new(db_path.to_str().unwrap())?;
    handler.add_indexes()?;

    let (messages, wxid_list) = handler.get_msg_list_with_users(
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

    let micro_db_path = db_path.parent()
        .and_then(|p| p.parent())
        .map(|p| p.join("MicroMsg.db"));
    
    let mut user_map = HashMap::new();
    if let Some(micro_path) = micro_db_path {
        if micro_path.exists() {
            let contact_handler = ContactHandler::new(micro_path.to_str().unwrap()).ok();
            if let Some(handler) = contact_handler {
                for wxid in &wxid_list {
                    if let Ok(Some(contact)) = handler.get_contact(wxid) {
                        user_map.insert(wxid.clone(), serde_json::json!({
                            "wxid": contact.wxid,
                            "nickname": contact.nickname,
                            "remark": contact.remark,
                            "head_img_url": contact.head_img_url,
                        }));
                    }
                }
            }
        }
    }

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
        user_list: user_map,
    }))
}

pub async fn get_contact_detail(
    Path(wxid): Path<String>,
    Json(req): Json<ChatContactsRequest>
) -> Result<Json<serde_json::Value>> {
    let db_path = PathBuf::from(&req.merge_path);
    if !db_path.exists() {
        return Err(AppError::NotFound(format!("Database not found: {}", req.merge_path)).into());
    }

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

    Ok(Json(serde_json::json!({
        "wxid": wxid,
        "nickname": None::<String>,
        "remark": None::<String>,
    })))
}

