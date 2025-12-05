use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FavoriteListRequest {
    pub merge_path: String,
    pub start: i64,
    pub limit: i64,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FavoriteListResponse {
    pub favorites: Vec<FavoriteItemResponse>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FavoriteItemResponse {
    pub id: i64,
    pub local_id: i64,
    pub talker: String,
    pub content: String,
    pub create_time: i64,
    pub create_time_str: String,
    pub msg_type: i32,
    pub type_name: String,
    pub src: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FavoriteCountRequest {
    pub merge_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FavoriteCountResponse {
    pub count: i64,
}

