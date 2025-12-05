use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactStatRequest {
    pub merge_path: String,
    pub wxid: String,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactStatResponse {
    pub wxid: String,
    pub total_count: i64,
    pub sender_count: i64,
    pub receiver_count: i64,
    pub date_stats: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateChatStatRequest {
    pub merge_path: String,
    pub wxid: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateChatStatResponse {
    pub stats: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateHeatmapRequest {
    pub merge_path: String,
    pub wxid: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateHeatmapResponse {
    pub data: Vec<HeatmapData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeatmapData {
    pub date: String,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopTalkersRequest {
    pub merge_path: String,
    pub top: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopTalkersResponse {
    pub talkers: HashMap<String, serde_json::Value>,
}

