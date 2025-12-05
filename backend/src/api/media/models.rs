use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaInfoRequest {
    pub media_db_path: String,
    pub msg_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaListRequest {
    pub media_db_path: String,
    pub wxid: String,
    pub media_type: Option<i32>,
    pub limit: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaInfoResponse {
    pub msg_id: i64,
    pub media_path: Option<String>,
    pub thumb_path: Option<String>,
    pub file_size: Option<i64>,
    pub media_type: Option<i32>,
    pub create_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaListResponse {
    pub media_list: Vec<MediaInfoResponse>,
}

