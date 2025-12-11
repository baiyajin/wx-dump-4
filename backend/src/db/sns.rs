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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use rusqlite::Connection;

    fn create_test_db() -> (TempDir, String) {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test_sns.db");
        let db_path_str = db_path.to_str().unwrap().to_string();
        
        let conn = Connection::open(&db_path).unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS FeedsV20 (
                FeedId INTEGER PRIMARY KEY,
                CreateTime INTEGER,
                UserName TEXT,
                Content TEXT,
                Type INTEGER
            )",
            [],
        ).unwrap();
        
        conn.execute(
            "INSERT INTO FeedsV20 (FeedId, CreateTime, UserName, Content, Type) 
             VALUES (?, ?, ?, ?, ?)",
            rusqlite::params![1, 1234567890, "test_user", "Test moment content", 1],
        ).unwrap();
        
        (temp_dir, db_path_str)
    }

    #[test]
    fn test_sns_handler_new() {
        let (_temp_dir, db_path) = create_test_db();
        let handler = SnsHandler::new(&db_path);
        assert!(handler.is_ok());
    }

    #[test]
    fn test_get_moments_list() {
        let (_temp_dir, db_path) = create_test_db();
        let handler = SnsHandler::new(&db_path).unwrap();
        let moments = handler.get_moments_list(0, 10, None, None).unwrap();
        assert!(!moments.is_empty());
    }

    #[test]
    fn test_get_moments_count() {
        let (_temp_dir, db_path) = create_test_db();
        let handler = SnsHandler::new(&db_path).unwrap();
        let count = handler.get_moments_count().unwrap();
        assert_eq!(count, 1);
    }
}

