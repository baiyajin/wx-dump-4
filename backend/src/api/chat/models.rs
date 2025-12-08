use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    pub user_list: HashMap<String, serde_json::Value>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MsgSearchRequest {
    pub merge_path: String,
    pub keyword: String,
    pub wxid: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MsgSearchResponse {
    pub messages: Vec<MessageResponse>,
    pub total: i64,
    pub keyword: String,
}

