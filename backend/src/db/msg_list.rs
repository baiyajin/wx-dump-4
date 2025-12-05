use crate::db::dbbase::DatabaseBase;
use crate::db::utils::{Message, timestamp_to_string};
use crate::db::msg_parser::MessageParser;
use crate::utils::Result;

/// 消息列表查询功能
pub struct MsgList {
    db: DatabaseBase,
}

impl MsgList {
    pub fn new(db: DatabaseBase) -> Self {
        Self { db }
    }

    /// 获取消息列表（带用户信息）
    pub fn get_msg_list_with_users(
        &self,
        wxid: Option<&str>,
        start_index: i64,
        page_size: i64,
        start_time: Option<i64>,
        end_time: Option<i64>,
    ) -> Result<(Vec<Message>, Vec<String>)> {
        let messages = self.get_msg_list(wxid, start_index, page_size, start_time, end_time)?;
        
        // 提取所有talker
        let mut wxid_set = std::collections::HashSet::new();
        for msg in &messages {
            wxid_set.insert(msg.talker.clone());
            wxid_set.insert(msg.str_talker.clone());
        }
        
        let wxid_list: Vec<String> = wxid_set.into_iter().collect();
        Ok((messages, wxid_list))
    }

    /// 获取消息列表
    pub fn get_msg_list(
        &self,
        wxid: Option<&str>,
        start_index: i64,
        page_size: i64,
        start_time: Option<i64>,
        end_time: Option<i64>,
    ) -> Result<Vec<Message>> {
        if !self.db.table_exists("MSG") {
            return Ok(Vec::new());
        }

        let mut sql = String::from(
            "SELECT localId, MsgSvrID, Type, SubType, CreateTime, IsSender, 
                    TalkerId, StrTalker, StrContent, DisplayContent, BytesExtra, CompressContent
             FROM MSG WHERE 1=1"
        );

        let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();

        if let Some(wxid) = wxid {
            sql.push_str(" AND StrTalker = ?");
            params.push(&wxid);
        }

        if let Some(start) = start_time {
            sql.push_str(" AND CreateTime >= ?");
            params.push(&start);
        }

        if let Some(end) = end_time {
            sql.push_str(" AND CreateTime <= ?");
            params.push(&end);
        }

        sql.push_str(" ORDER BY CreateTime ASC LIMIT ? OFFSET ?");
        params.push(&page_size);
        params.push(&start_index);

        let messages = self.db.execute_query(&sql, &params, |row| {
            let msg_type: i32 = row.get(2)?;
            let sub_type: i32 = row.get(3)?;
            let create_time: i64 = row.get(4)?;
            let content: String = row.get(8)?;
            let bytes_extra: Option<Vec<u8>> = row.get(10).ok();
            let compress_content: Option<Vec<u8>> = row.get(11).ok();

            // 解析消息内容
            let (parsed_content, src, extra) = Self::parse_message_content(
                msg_type,
                sub_type,
                &content,
                bytes_extra.as_deref(),
                compress_content.as_deref(),
            );

            Ok(Message {
                id: 0, // 将在后续计算
                local_id: row.get(0)?,
                msg_svr_id: row.get(1)?,
                msg_type,
                sub_type,
                type_name: crate::db::utils::get_message_type_name(msg_type, sub_type).to_string(),
                create_time,
                create_time_str: timestamp_to_string(create_time),
                is_sender: row.get(5)?,
                talker: row.get(6)?,
                str_talker: row.get(7)?,
                content: parsed_content,
                display_content: row.get(9)?,
                src,
                extra,
            })
        })?;

        // 为消息分配ID
        let mut messages_with_id = Vec::new();
        for (idx, mut msg) in messages.into_iter().enumerate() {
            msg.id = (start_index + idx as i64) as i64;
            messages_with_id.push(msg);
        }

        Ok(messages_with_id)
    }

    /// 解析消息内容
    fn parse_message_content(
        msg_type: i32,
        sub_type: i32,
        content: &str,
        bytes_extra: Option<&[u8]>,
        compress_content: Option<&[u8]>,
    ) -> (String, String, serde_json::Value) {
        match (msg_type, sub_type) {
            (1, 0) => {
                // 文本消息
                (content.to_string(), String::new(), serde_json::json!({}))
            }
            (3, 0) => {
                // 图片消息
                let src = bytes_extra
                    .and_then(|b| MessageParser::parse_image_path(b))
                    .unwrap_or_default();
                ("图片".to_string(), src, serde_json::json!({}))
            }
            (34, 0) => {
                // 语音消息
                let voice_info = MessageParser::parse_voice_message(content);
                let voicelength = voice_info.get("voicelength").cloned().unwrap_or_default();
                let transtext = voice_info.get("transtext").cloned().unwrap_or_default();
                let msg = if !transtext.is_empty() {
                    format!("语音时长：{}秒\n翻译结果：{}", voicelength, transtext)
                } else {
                    format!("语音时长：{}秒", voicelength)
                };
                (msg, String::new(), serde_json::json!(voice_info))
            }
            (43, 0) => {
                // 视频消息
                let src = bytes_extra
                    .and_then(|b| MessageParser::parse_video_message(b))
                    .unwrap_or_default();
                ("视频".to_string(), src, serde_json::json!({}))
            }
            (47, 0) => {
                // 动画表情
                let cdnurl = MessageParser::parse_emoji_message(content, bytes_extra.unwrap_or(&[]))
                    .unwrap_or_default();
                ("表情".to_string(), cdnurl, serde_json::json!({}))
            }
            (48, 0) => {
                // 位置消息
                let location = MessageParser::parse_location_message(content);
                let msg = format!(
                    "纬度:【{}】 经度:【{}】\n位置：{} {}",
                    location.get("latitude").unwrap_or(&String::new()),
                    location.get("longitude").unwrap_or(&String::new()),
                    location.get("label").unwrap_or(&String::new()),
                    location.get("poiname").unwrap_or(&String::new())
                );
                (msg, String::new(), serde_json::json!(location))
            }
            (49, 0) | (49, 6) => {
                // 文件消息
                let src = bytes_extra
                    .and_then(|b| MessageParser::parse_file_message(b))
                    .unwrap_or_default();
                let file_name = std::path::Path::new(&src)
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("文件")
                    .to_string();
                (file_name, src, serde_json::json!({}))
            }
            (49, 5) => {
                // 分享卡片式链接
                let share_info = MessageParser::parse_share_message(
                    compress_content.unwrap_or(&[]),
                    content,
                );
                let title = share_info.get("title")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let des = share_info.get("des")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let url = share_info.get("url")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let msg = format!("{}\n{}\n\n链接：{}", title, des, url);
                (msg, url, serde_json::json!(share_info))
            }
            _ => {
                // 其他类型
                (content.to_string(), String::new(), serde_json::json!({}))
            }
        }
    }
}

