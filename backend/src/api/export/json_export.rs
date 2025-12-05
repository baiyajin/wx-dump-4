use crate::db::msg::MsgHandler;
use crate::utils::Result;
use std::fs::File;
use std::io::Write;
use serde_json::json;

pub struct JsonExporter;

impl JsonExporter {
    pub fn export(
        handler: &MsgHandler,
        wxid: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        output_path: &str,
    ) -> Result<String> {
        let mut messages = Vec::new();
        let mut start_index = 0;
        let page_size = 1000;

        loop {
            let (msgs, _) = handler.get_msg_list_with_users(
                wxid,
                start_index,
                page_size,
                start_time,
                end_time,
            )?;

            if msgs.is_empty() {
                break;
            }

            for msg in msgs {
                messages.push(json!({
                    "id": msg.id,
                    "local_id": msg.local_id,
                    "msg_svr_id": msg.msg_svr_id,
                    "msg_type": msg.msg_type,
                    "sub_type": msg.sub_type,
                    "type_name": msg.type_name,
                    "create_time": msg.create_time,
                    "create_time_str": msg.create_time_str,
                    "is_sender": msg.is_sender,
                    "talker": msg.talker,
                    "str_talker": msg.str_talker,
                    "content": msg.content,
                    "display_content": msg.display_content,
                    "src": msg.src,
                    "extra": msg.extra,
                }));
            }

            start_index += page_size;
        }

        let json_data = json!({
            "total": messages.len(),
            "messages": messages,
        });

        let json_string = serde_json::to_string_pretty(&json_data)?;
        let mut file = File::create(output_path)?;
        file.write_all(json_string.as_bytes())?;

        Ok(format!("成功导出 {} 条消息到 {}", messages.len(), output_path))
    }
}

