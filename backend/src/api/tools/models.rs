use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct GetBiasRequest {
    pub version: String,
}

#[derive(Debug, Serialize)]
pub struct GetBiasResponse {
    pub success: bool,
    pub message: String,
    pub bias: Option<Vec<u32>>,
}

#[derive(Debug, Deserialize)]
pub struct DecryptDbRequest {
    pub key: String,
    pub db_path: String,
    pub out_path: String,
}

#[derive(Debug, Serialize)]
pub struct DecryptDbResponse {
    pub success: bool,
    pub message: String,
    pub out_path: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MergeDbRequest {
    pub db_paths: Vec<String>,
    pub out_path: String,
}

#[derive(Debug, Serialize)]
pub struct MergeDbResponse {
    pub success: bool,
    pub message: String,
    pub out_path: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct WxInfoToolResponse {
    pub success: bool,
    pub message: String,
    pub info: Option<serde_json::Value>,
}

