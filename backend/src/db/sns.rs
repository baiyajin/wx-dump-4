use crate::db::dbbase::DatabaseBase;
use crate::utils::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MomentItem {
    pub feed_id: i64,
    pub create_time: i64,
    pub create_time_str: String,
    pub user_name: String,
    pub content: String,
    pub feed_type: i32,
}

pub struct SnsHandler {
    db: DatabaseBase,
}

impl SnsHandler {
    pub fn new(db_path: &str) -> Result<Self> {
        let db = DatabaseBase::new(db_path)?;
        Ok(Self { db })
    }

    /// 获取朋友圈列表
    pub fn get_moments_list(
        &self,
        start_index: i64,
        page_size: i64,
        start_time: Option<i64>,
        end_time: Option<i64>,
    ) -> Result<Vec<MomentItem>> {
        if !self.db.table_exists("FeedsV20") {
            return Ok(Vec::new());
        }

        let mut sql = String::from(
            "SELECT FeedId, CreateTime, UserName, Content, Type
             FROM FeedsV20 WHERE 1=1"
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

        sql.push_str(" ORDER BY CreateTime DESC LIMIT ? OFFSET ?");
        params.push(&page_size);
        params.push(&start_index);

        let moments = self.db.execute_query(&sql, &params, |row| {
            let create_time: i64 = row.get(1)?;
            
            Ok(MomentItem {
                feed_id: row.get(0)?,
                create_time,
                create_time_str: crate::db::utils::timestamp_to_string(create_time),
                user_name: row.get(2)?,
                content: row.get(3)?,
                feed_type: row.get(4)?,
            })
        })?;

        Ok(moments)
    }

    /// 获取朋友圈数量
    pub fn get_moments_count(&self) -> Result<i64> {
        if !self.db.table_exists("FeedsV20") {
            return Ok(0);
        }

        let count: i64 = self.db.execute_query(
            "SELECT COUNT(*) FROM FeedsV20",
            &[],
            |row| Ok(row.get::<_, i64>(0)?)
        )?.first().copied().unwrap_or(0);

        Ok(count)
    }
}

