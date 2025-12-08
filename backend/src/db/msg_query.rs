use crate::db::dbbase::DatabaseBase;
use crate::utils::{Result, cache::CacheManager};
use std::collections::HashMap;
use std::sync::Arc;

/// 消息查询相关功能
pub struct MsgQuery {
    db: DatabaseBase,
}

impl MsgQuery {
    pub fn new(db: DatabaseBase) -> Self {
        Self { db }
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
    pub fn get_top_talkers(
        &self,
        top: i64,
        start_time: Option<i64>,
        end_time: Option<i64>,
    ) -> Result<HashMap<String, serde_json::Value>> {
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

