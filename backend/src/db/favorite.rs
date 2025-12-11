use crate::db::dbbase::DatabaseBase;
use crate::utils::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FavoriteItem {
    pub id: i64,
    pub local_id: i64,
    pub talker: String,
    pub content: String,
    pub create_time: i64,
    pub create_time_str: String,
    pub msg_type: i32,
    pub type_name: String,
    pub src: String,
}

pub struct FavoriteHandler {
    db: DatabaseBase,
}

impl FavoriteHandler {
    pub fn new(db_path: &str) -> Result<Self> {
        let db = DatabaseBase::new(db_path)?;
        Ok(Self { db })
    }

    /// 获取收藏列表
    pub fn get_favorite_list(
        &self,
        start_index: i64,
        page_size: i64,
        start_time: Option<i64>,
        end_time: Option<i64>,
    ) -> Result<Vec<FavoriteItem>> {
        if !self.db.table_exists("Favorite") {
            return Ok(Vec::new());
        }

        let mut sql = String::from(
            "SELECT localId, TalkerId, Content, CreateTime, Type, Reserved0
             FROM Favorite WHERE 1=1"
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

        let favorites = self.db.execute_query(&sql, &params, |row| {
            let msg_type: i32 = row.get(4)?;
            let create_time: i64 = row.get(3)?;
            
            Ok(FavoriteItem {
                id: 0,
                local_id: row.get(0)?,
                talker: row.get(1)?,
                content: row.get(2)?,
                create_time,
                create_time_str: crate::db::utils::timestamp_to_string(create_time),
                msg_type,
                type_name: crate::db::utils::get_message_type_name(msg_type, 0).to_string(),
                src: String::new(),
            })
        })?;

        // 为收藏项分配ID
        let mut favorites_with_id = Vec::new();
        for (idx, mut fav) in favorites.into_iter().enumerate() {
            fav.id = (start_index + idx as i64) as i64;
            favorites_with_id.push(fav);
        }

        Ok(favorites_with_id)
    }

    /// 获取收藏数量
    pub fn get_favorite_count(&self) -> Result<i64> {
        if !self.db.table_exists("Favorite") {
            return Ok(0);
        }

        let count: i64 = self.db.execute_query(
            "SELECT COUNT(*) FROM Favorite",
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
        let db_path = temp_dir.path().join("test_favorite.db");
        let db_path_str = db_path.to_str().unwrap().to_string();
        
        let conn = Connection::open(&db_path).unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS Favorite (
                localId INTEGER PRIMARY KEY,
                TalkerId TEXT,
                Content TEXT,
                CreateTime INTEGER,
                Type INTEGER,
                Reserved0 TEXT
            )",
            [],
        ).unwrap();
        
        conn.execute(
            "INSERT INTO Favorite (TalkerId, Content, CreateTime, Type) 
             VALUES (?, ?, ?, ?)",
            rusqlite::params!["test_wxid", "Test favorite content", 1234567890, 1],
        ).unwrap();
        
        (temp_dir, db_path_str)
    }

    #[test]
    fn test_favorite_handler_new() {
        let (_temp_dir, db_path) = create_test_db();
        let handler = FavoriteHandler::new(&db_path);
        assert!(handler.is_ok());
    }

    #[test]
    fn test_get_favorite_list() {
        let (_temp_dir, db_path) = create_test_db();
        let handler = FavoriteHandler::new(&db_path).unwrap();
        let favorites = handler.get_favorite_list(0, 10, None, None).unwrap();
        assert!(!favorites.is_empty());
    }

    #[test]
    fn test_get_favorite_count() {
        let (_temp_dir, db_path) = create_test_db();
        let handler = FavoriteHandler::new(&db_path).unwrap();
        let count = handler.get_favorite_count().unwrap();
        assert_eq!(count, 1);
    }
}

