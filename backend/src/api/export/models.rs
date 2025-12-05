use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportRequest {
    pub merge_path: String,
    pub wxid: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub output_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportResponse {
    pub success: bool,
    pub message: String,
    pub file_path: Option<String>,
}

