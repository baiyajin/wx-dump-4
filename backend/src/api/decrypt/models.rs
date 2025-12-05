use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecryptRequest {
    pub db_path: String,
    pub key: String,
    pub output_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecryptResponse {
    pub success: bool,
    pub message: String,
    pub decrypted_path: Option<String>,
}

