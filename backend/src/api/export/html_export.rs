use crate::db::msg::MsgHandler;
use crate::utils::Result;
use std::fs::File;
use std::io::Write;

pub struct HtmlExporter;

impl HtmlExporter {
    pub fn export(
        handler: &MsgHandler,
        wxid: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        output_path: &str,
    ) -> Result<String> {
        let mut file = File::create(output_path)?;
        
        // 写入HTML头部
        writeln!(file, r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>微信聊天记录导出</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 20px; background: #f5f5f5; }}
        .message {{ background: white; margin: 10px 0; padding: 15px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }}
        .message-header {{ color: #666; font-size: 12px; margin-bottom: 8px; }}
        .message-content {{ color: #333; line-height: 1.6; }}
        .message-sender {{ color: #07c160; font-weight: bold; }}
        .message-receiver {{ color: #576b95; }}
        .message-time {{ color: #999; }}
    </style>
</head>
<body>
    <h1>微信聊天记录</h1>
"#)?;

        let mut start_index = 0;
        let page_size = 1000;
        let mut total_exported = 0;

        loop {
            let (messages, _) = handler.get_msg_list_with_users(
                wxid,
                start_index,
                page_size,
                start_time,
                end_time,
            )?;

            if messages.is_empty() {
                break;
            }

            for msg in messages {
                let talker_class = if msg.is_sender == 1 {
                    "message-sender"
                } else {
                    "message-receiver"
                };

                writeln!(
                    file,
                    r#"    <div class="message">
        <div class="message-header">
            <span class="{}">{}</span>
            <span class="message-time"> - {}</span>
        </div>
        <div class="message-content">{}</div>
    </div>"#,
                    talker_class,
                    if msg.is_sender == 1 { "我" } else { &msg.talker },
                    msg.create_time_str,
                    html_escape(&msg.content)
                )?;

                total_exported += 1;
            }

            start_index += page_size;
        }

        writeln!(file, r#"
</body>
</html>"#)?;

        Ok(format!("成功导出 {} 条消息到 {}", total_exported, output_path))
    }
}

fn html_escape(text: &str) -> String {
    text.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&#39;")
}

