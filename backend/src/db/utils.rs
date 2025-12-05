use chrono::{DateTime, Local, TimeZone};
use serde::{Deserialize, Serialize};

/// 消息类型映射
pub fn get_message_type_name(msg_type: i32, sub_type: i32) -> &'static str {
    match (msg_type, sub_type) {
        (1, 0) => "文本",
        (3, 0) => "图片",
        (34, 0) => "语音",
        (37, 0) => "添加好友",
        (42, 0) => "推荐公众号",
        (43, 0) => "视频",
        (47, 0) => "动画表情",
        (48, 0) => "位置",
        (49, 0) => "文件",
        (49, 1) => "粘贴的文本",
        (49, 3) => "(分享)音乐",
        (49, 4) => "(分享)卡片式链接",
        (49, 5) => "(分享)卡片式链接",
        (49, 6) => "文件",
        (49, 7) => "游戏相关",
        (49, 8) => "用户上传的GIF表情",
        (49, 15) => "未知-49,15",
        (49, 17) => "位置共享",
        (49, 19) => "合并转发的聊天记录",
        (49, 24) => "(分享)笔记",
        (49, 33) => "(分享)小程序",
        (49, 36) => "(分享)小程序",
        (49, 40) => "(分享)收藏夹",
        (49, 44) => "(分享)小说(猜)",
        (49, 50) => "(分享)视频号名片",
        (49, 51) => "(分享)视频号视频",
        (49, 53) => "接龙",
        (49, 57) => "引用回复",
        (49, 63) => "视频号直播或直播回放",
        (49, 74) => "文件(猜)",
        (49, 87) => "群公告",
        (49, 88) => "视频号直播或直播回放等",
        (49, 2000) => "转账",
        (49, 2003) => "赠送红包封面",
        (50, 0) => "语音通话",
        (62, 0) => "小视频",
        (65, 0) => "企业微信打招呼(猜)",
        (66, 0) => "企业微信添加好友(猜)",
        (10000, 0) => "系统通知",
        (10000, 1) => "消息撤回1",
        (10000, 4) => "拍一拍",
        (10000, 5) => "消息撤回5",
        (10000, 6) => "消息撤回6",
        (10000, 33) => "消息撤回33",
        (10000, 36) => "消息撤回36",
        (10000, 57) => "消息撤回57",
        (10000, 8000) => "邀请加群",
        (11000, 0) => "未知-11000,0",
        _ => "未知",
    }
}

/// 时间戳转字符串
pub fn timestamp_to_string(timestamp: i64) -> String {
    if let Some(dt) = Local.timestamp_opt(timestamp, 0).single() {
        dt.format("%Y-%m-%d %H:%M:%S").to_string()
    } else {
        "1970-01-01 00:00:00".to_string()
    }
}

/// 字符串转时间戳
pub fn string_to_timestamp(time_str: &str) -> Option<i64> {
    if let Ok(dt) = DateTime::parse_from_str(time_str, "%Y-%m-%d %H:%M:%S") {
        Some(dt.timestamp())
    } else {
        None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: i64,
    pub local_id: i64,
    pub msg_svr_id: i64,
    pub msg_type: i32,
    pub sub_type: i32,
    pub type_name: String,
    pub create_time: i64,
    pub create_time_str: String,
    pub is_sender: i32,
    pub talker: String,
    pub str_talker: String,
    pub content: String,
    pub display_content: String,
    pub src: String,
    pub extra: serde_json::Value,
}

impl Message {
    pub fn from_row(row: &rusqlite::Row) -> rusqlite::Result<Self> {
        let msg_type: i32 = row.get(3)?;
        let sub_type: i32 = row.get(4)?;
        let create_time: i64 = row.get(5)?;
        
        Ok(Self {
            id: row.get(0)?,
            local_id: row.get(1)?,
            msg_svr_id: row.get(2)?,
            msg_type,
            sub_type,
            type_name: get_message_type_name(msg_type, sub_type).to_string(),
            create_time,
            create_time_str: timestamp_to_string(create_time),
            is_sender: row.get(6)?,
            talker: row.get(7)?,
            str_talker: row.get(8)?,
            content: row.get(9)?,
            display_content: row.get(10)?,
            src: String::new(),
            extra: serde_json::json!({}),
        })
    }
}

