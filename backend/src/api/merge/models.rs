use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeRequest {
    pub source_paths: Vec<String>,
    pub output_path: String,
    pub remove_duplicates: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeResponse {
    pub success: bool,
    pub message: String,
    pub total_inserted: Option<usize>,
}

