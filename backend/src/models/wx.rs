use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WxInfoResponse {
    pub pid: u32,
    pub version: String,
    pub account: Option<String>,
    pub mobile: Option<String>,
    pub nickname: Option<String>,
    pub mail: Option<String>,
    pub wxid: Option<String>,
    pub key: Option<String>,
    pub wx_dir: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecryptRequest {
    pub key: String,
    pub db_path: String,
    pub out_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecryptResponse {
    pub success: bool,
    pub message: String,
    pub out_path: Option<String>,
}

