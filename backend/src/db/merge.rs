use crate::db::dbbase::DatabaseBase;
use crate::utils::Result;
use anyhow::Context;
use rusqlite::params;
use std::collections::HashSet;
use std::path::Path;

pub struct DbMerger;

impl DbMerger {
    /// 合并多个MSG数据库
    pub fn merge_databases(
        source_paths: &[String],
        output_path: &str,
        remove_duplicates: bool,
    ) -> Result<usize> {
        if source_paths.is_empty() {
            return Err(anyhow::anyhow!("No source databases provided").into());
        }

        // 检查所有源数据库是否存在
        for path in source_paths {
            if !Path::new(path).exists() {
                return Err(anyhow::anyhow!("Source database not found: {}", path).into());
            }
        }

        // 创建输出数据库
        let output_db = DatabaseBase::new(output_path)?;
        
        // 创建MSG表（如果不存在）
        Self::create_msg_table(&output_db)?;

        let mut total_inserted = 0;
        let mut seen_ids = HashSet::new();

        for source_path in source_paths {
            let source_db = DatabaseBase::new(source_path)?;
            
            if !source_db.table_exists("MSG") {
                continue;
            }

            let messages = Self::read_all_messages(&source_db)?;

            for msg in messages {
                // 检查是否重复（基于MsgSvrID）
                if remove_duplicates {
                    if seen_ids.contains(&msg.msg_svr_id) {
                        continue;
                    }
                    seen_ids.insert(msg.msg_svr_id);
                }

                // 插入消息
                Self::insert_message(&output_db, &msg)?;
                total_inserted += 1;
            }
        }

        // 创建索引
        Self::create_indexes(&output_db)?;

        Ok(total_inserted)
    }

    fn create_msg_table(db: &DatabaseBase) -> Result<()> {
        let sql = r#"
            CREATE TABLE IF NOT EXISTS MSG (
                localId INTEGER PRIMARY KEY,
                TalkerId TEXT,
                MsgSvrID INTEGER,
                Type INTEGER,
                SubType INTEGER,
                CreateTime INTEGER,
                IsSender INTEGER,
                Sequence INTEGER,
                StatusEx INTEGER,
                FlagEx INTEGER,
                Status INTEGER,
                MsgSequence INTEGER,
                StrContent TEXT,
                MsgServerSeq INTEGER,
                StrTalker TEXT,
                DisplayContent TEXT,
                Reserved0 INTEGER,
                Reserved1 INTEGER,
                Reserved3 INTEGER,
                Reserved4 INTEGER,
                Reserved5 INTEGER,
                Reserved6 INTEGER,
                CompressContent BLOB,
                BytesExtra BLOB,
                BytesTrans BLOB,
                Reserved2 INTEGER
            )
        "#;

        db.execute_batch(sql)?;
        Ok(())
    }

    fn read_all_messages(db: &DatabaseBase) -> Result<Vec<MessageRow>> {
        let sql = "SELECT localId, TalkerId, MsgSvrID, Type, SubType, CreateTime, IsSender,
                          Sequence, StatusEx, FlagEx, Status, MsgSequence, StrContent,
                          MsgServerSeq, StrTalker, DisplayContent, Reserved0, Reserved1,
                          Reserved3, Reserved4, Reserved5, Reserved6, CompressContent,
                          BytesExtra, BytesTrans, Reserved2
                   FROM MSG
                   ORDER BY CreateTime ASC";

        let messages = db.execute_query(sql, &[], |row| {
            Ok(MessageRow {
                local_id: row.get(0)?,
                talker_id: row.get(1)?,
                msg_svr_id: row.get(2)?,
                msg_type: row.get(3)?,
                sub_type: row.get(4)?,
                create_time: row.get(5)?,
                is_sender: row.get(6)?,
                sequence: row.get(7)?,
                status_ex: row.get(8)?,
                flag_ex: row.get(9)?,
                status: row.get(10)?,
                msg_sequence: row.get(11)?,
                str_content: row.get(12)?,
                msg_server_seq: row.get(13)?,
                str_talker: row.get(14)?,
                display_content: row.get(15)?,
                reserved0: row.get(16).ok(),
                reserved1: row.get(17).ok(),
                reserved3: row.get(18).ok(),
                reserved4: row.get(19).ok(),
                reserved5: row.get(20).ok(),
                reserved6: row.get(21).ok(),
                compress_content: row.get(22).ok(),
                bytes_extra: row.get(23).ok(),
                bytes_trans: row.get(24).ok(),
                reserved2: row.get(25).ok(),
            })
        })?;

        Ok(messages)
    }

    fn insert_message(db: &DatabaseBase, msg: &MessageRow) -> Result<()> {
        let sql = r#"
            INSERT INTO MSG (
                localId, TalkerId, MsgSvrID, Type, SubType, CreateTime, IsSender,
                Sequence, StatusEx, FlagEx, Status, MsgSequence, StrContent,
                MsgServerSeq, StrTalker, DisplayContent, Reserved0, Reserved1,
                Reserved3, Reserved4, Reserved5, Reserved6, CompressContent,
                BytesExtra, BytesTrans, Reserved2
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#;

        db.execute(sql, &[
            &msg.local_id,
            &msg.talker_id,
            &msg.msg_svr_id,
            &msg.msg_type,
            &msg.sub_type,
            &msg.create_time,
            &msg.is_sender,
            &msg.sequence,
            &msg.status_ex,
            &msg.flag_ex,
            &msg.status,
            &msg.msg_sequence,
            &msg.str_content,
            &msg.msg_server_seq,
            &msg.str_talker,
            &msg.display_content,
            &msg.reserved0,
            &msg.reserved1,
            &msg.reserved3,
            &msg.reserved4,
            &msg.reserved5,
            &msg.reserved6,
            &msg.compress_content.as_deref(),
            &msg.bytes_extra.as_deref(),
            &msg.bytes_trans.as_deref(),
            &msg.reserved2,
        ])?;

        Ok(())
    }

    fn create_indexes(db: &DatabaseBase) -> Result<()> {
        db.execute_batch(
            "CREATE INDEX IF NOT EXISTS idx_MSG_StrTalker ON MSG(StrTalker);
             CREATE INDEX IF NOT EXISTS idx_MSG_CreateTime ON MSG(CreateTime);
             CREATE INDEX IF NOT EXISTS idx_MSG_MsgSvrID ON MSG(MsgSvrID);
             CREATE INDEX IF NOT EXISTS idx_MSG_StrTalker_CreateTime ON MSG(StrTalker, CreateTime);"
        )?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct MessageRow {
    local_id: i64,
    talker_id: String,
    msg_svr_id: i64,
    msg_type: i32,
    sub_type: i32,
    create_time: i64,
    is_sender: i32,
    sequence: i64,
    status_ex: i64,
    flag_ex: i64,
    status: i64,
    msg_sequence: i64,
    str_content: String,
    msg_server_seq: i64,
    str_talker: String,
    display_content: String,
    reserved0: Option<i64>,
    reserved1: Option<i64>,
    reserved3: Option<i64>,
    reserved4: Option<i64>,
    reserved5: Option<i64>,
    reserved6: Option<i64>,
    compress_content: Option<Vec<u8>>,
    bytes_extra: Option<Vec<u8>>,
    bytes_trans: Option<Vec<u8>>,
    reserved2: Option<i64>,
}

