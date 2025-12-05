use crate::db::msg::MsgHandler;
use crate::utils::Result;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub struct CsvExporter;

impl CsvExporter {
    pub fn export(
        handler: &MsgHandler,
        wxid: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
        output_path: &str,
    ) -> Result<String> {
        let mut file = File::create(output_path)?;
        
        // 写入CSV头部
        writeln!(file, "ID,时间,发送者,消息类型,内容,文件路径")?;

        let mut start_index = 0;
        let page_size = 1000;
        let mut total_exported = 0;

        loop {
            let messages = handler.get_msg_list_with_users(
                wxid,
                start_index,
                page_size,
                start_time,
                end_time,
            )?;

            if messages.0.is_empty() {
                break;
            }

            for msg in messages.0 {
                let talker = if msg.is_sender == 1 {
                    "我".to_string()
                } else {
                    msg.talker.clone()
                };

                writeln!(
                    file,
                    "{},{},{},{},{},\"{}\"",
                    msg.id,
                    msg.create_time_str,
                    talker,
                    msg.type_name,
                    msg.content.replace("\"", "\"\""),
                    msg.src
                )?;

                total_exported += 1;
            }

            start_index += page_size;
        }

        Ok(format!("成功导出 {} 条消息到 {}", total_exported, output_path))
    }
}

