use crate::db::dbbase::DatabaseBase;
use crate::db::utils::{Message, timestamp_to_string};
use crate::utils::Result;
use anyhow::Context;
use rusqlite::params;

pub struct MsgHandler {
    db: DatabaseBase,
}

impl MsgHandler {
    pub fn new(db_path: &str) -> Result<Self> {
        let db = DatabaseBase::new(db_path)?;
        Ok(Self { db })
    }

    /// 添加索引以加快查询速度
    pub fn add_indexes(&self) -> Result<()> {
        if !self.db.table_exists("MSG") {
            return Ok(());
        }

        self.db.execute_batch(
            "CREATE INDEX IF NOT EXISTS idx_MSG_StrTalker ON MSG(StrTalker);
             CREATE INDEX IF NOT EXISTS idx_MSG_CreateTime ON MSG(CreateTime);
             CREATE INDEX IF NOT EXISTS idx_MSG_StrTalker_CreateTime ON MSG(StrTalker, CreateTime);"
        )?;

        Ok(())
    }

    /// 获取消息数量
    pub fn get_msg_count(&self, wxid: Option<&str>) -> Result<HashMap<String, i64>> {
        if !self.db.table_exists("MSG") {
            return Ok(HashMap::new());
        }

        let sql = if let Some(wxid) = wxid {
            "SELECT StrTalker, COUNT(*) FROM MSG WHERE StrTalker = ? GROUP BY StrTalker"
        } else {
            "SELECT StrTalker, COUNT(*) FROM MSG GROUP BY StrTalker ORDER BY COUNT(*) DESC"
        };

        let mut result = HashMap::new();
        let rows = if let Some(wxid) = wxid {
            self.db.execute_query(sql, &[&wxid], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
            })?
        } else {
            self.db.execute_query(sql, &[], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
            })?
        };

        for (talker, count) in rows {
            result.insert(talker, count);
        }

        // 获取总数
        let total: i64 = self.db.execute_query(
            "SELECT COUNT(*) FROM MSG",
            &[],
            |row| Ok(row.get::<_, i64>(0)?)
        )?.first().copied().unwrap_or(0);

        result.insert("total".to_string(), total);
        Ok(result)
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
                    TalkerId, StrTalker, StrContent, DisplayContent
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
                content: row.get(8)?,
                display_content: row.get(9)?,
                src: String::new(),
                extra: serde_json::json!({}),
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

    /// 获取日期聊天统计
    pub fn get_date_count(
        &self,
        wxid: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
    ) -> Result<HashMap<String, serde_json::Value>> {
        if !self.db.table_exists("MSG") {
            return Ok(HashMap::new());
        }

        let mut sql = String::from(
            "SELECT strftime('%Y-%m-%d', CreateTime, 'unixepoch', 'localtime') AS date,
                    COUNT(*) AS total_count,
                    SUM(CASE WHEN IsSender = 1 THEN 1 ELSE 0 END) AS sender_count,
                    SUM(CASE WHEN IsSender = 0 THEN 1 ELSE 0 END) AS receiver_count
             FROM MSG WHERE StrTalker NOT LIKE '%chatroom%'"
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

        sql.push_str(" GROUP BY date ORDER BY date ASC");

        let rows = self.db.execute_query(&sql, &params, |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, i64>(2)?,
                row.get::<_, i64>(3)?,
            ))
        })?;

        let mut result = HashMap::new();
        for (date, total, sender, receiver) in rows {
            result.insert(
                date,
                serde_json::json!({
                    "total_count": total,
                    "sender_count": sender,
                    "receiver_count": receiver,
                }),
            );
        }

        Ok(result)
    }

    /// 获取聊天最多的联系人
    pub fn get_top_talkers(&self, top: i64, start_time: Option<i64>, end_time: Option<i64>) -> Result<HashMap<String, serde_json::Value>> {
        if !self.db.table_exists("MSG") {
            return Ok(HashMap::new());
        }

        let mut sql = String::from(
            "SELECT StrTalker, COUNT(*) AS count,
                    SUM(CASE WHEN IsSender = 1 THEN 1 ELSE 0 END) AS sender_count,
                    SUM(CASE WHEN IsSender = 0 THEN 1 ELSE 0 END) AS receiver_count
             FROM MSG WHERE StrTalker NOT LIKE '%chatroom%'"
        );

        let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();

        if let Some(start) = start_time {
            sql.push_str(" AND CreateTime >= ?");
            params.push(&start);
        }

        if let Some(end) = end_time {
            sql.push_str(" AND CreateTime <= ?");
            params.push(&end);
        }

        sql.push_str(" GROUP BY StrTalker ORDER BY count DESC LIMIT ?");
        params.push(&top);

        let rows = self.db.execute_query(&sql, &params, |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, i64>(2)?,
                row.get::<_, i64>(3)?,
            ))
        })?;

        let mut result = HashMap::new();
        for (talker, total, sender, receiver) in rows {
            result.insert(
                talker,
                serde_json::json!({
                    "total_count": total,
                    "sender_count": sender,
                    "receiver_count": receiver,
                }),
            );
        }

        Ok(result)
    }
}

