use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MomentsListRequest {
    pub merge_path: String,
    pub start: i64,
    pub limit: i64,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MomentsListResponse {
    pub moments: Vec<MomentItemResponse>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MomentItemResponse {
    pub feed_id: i64,
    pub create_time: i64,
    pub create_time_str: String,
    pub user_name: String,
    pub content: String,
    pub feed_type: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MomentsCountRequest {
    pub merge_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MomentsCountResponse {
    pub count: i64,
}

