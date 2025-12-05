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

