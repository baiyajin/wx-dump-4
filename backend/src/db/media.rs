use crate::db::dbbase::DatabaseBase;
use crate::utils::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaInfo {
    pub msg_id: i64,
    pub media_path: Option<String>,
    pub thumb_path: Option<String>,
    pub file_size: Option<i64>,
    pub media_type: Option<i32>,
    pub create_time: Option<i64>,
}

pub struct MediaHandler {
    db: DatabaseBase,
}

impl MediaHandler {
    pub fn new(db_path: &str) -> Result<Self> {
        let db = DatabaseBase::new(db_path)?;
        Ok(Self { db })
    }

    /// 获取媒体文件信息
    pub fn get_media_info(&self, msg_id: i64) -> Result<Option<MediaInfo>> {
        if !self.db.table_exists("Media") {
            return Ok(None);
        }

        let sql = "SELECT MsgSvrId, BigHeadImgUrl, SmallHeadImgUrl, Reserved0, Reserved1, Reserved2
                   FROM Media WHERE MsgSvrId = ?";

        let results = self.db.execute_query(sql, &[&msg_id], |row| {
            Ok(MediaInfo {
                msg_id: row.get(0)?,
                media_path: row.get(1).ok(),
                thumb_path: row.get(2).ok(),
                file_size: row.get(3).ok(),
                media_type: row.get(4).ok(),
                create_time: row.get(5).ok(),
            })
        })?;

        Ok(results.first().cloned())
    }

    /// 获取联系人媒体文件列表
    pub fn get_contact_media_list(
        &self,
        wxid: &str,
        media_type: Option<i32>,
        limit: Option<i64>,
    ) -> Result<Vec<MediaInfo>> {
        if !self.db.table_exists("Media") {
            return Ok(Vec::new());
        }

        let mut sql = String::from(
            "SELECT MsgSvrId, BigHeadImgUrl, SmallHeadImgUrl, Reserved0, Reserved1, Reserved2
             FROM Media WHERE 1=1"
        );

        let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();

        // 如果有MsgSvrId关联，可以通过MSG表关联查询
        // 这里简化处理，直接查询Media表
        if let Some(mtype) = media_type {
            sql.push_str(" AND Reserved1 = ?");
            params.push(&mtype);
        }

        sql.push_str(" ORDER BY Reserved2 DESC");

        if let Some(limit_val) = limit {
            sql.push_str(" LIMIT ?");
            params.push(&limit_val);
        }

        let results = self.db.execute_query(&sql, &params, |row| {
            Ok(MediaInfo {
                msg_id: row.get(0)?,
                media_path: row.get(1).ok(),
                thumb_path: row.get(2).ok(),
                file_size: row.get(3).ok(),
                media_type: row.get(4).ok(),
                create_time: row.get(5).ok(),
            })
        })?;

        Ok(results)
    }

    /// 获取图片列表
    pub fn get_image_list(&self, wxid: &str, limit: Option<i64>) -> Result<Vec<MediaInfo>> {
        self.get_contact_media_list(wxid, Some(3), limit)
    }

    /// 获取视频列表
    pub fn get_video_list(&self, wxid: &str, limit: Option<i64>) -> Result<Vec<MediaInfo>> {
        self.get_contact_media_list(wxid, Some(43), limit)
    }

    /// 获取文件列表
    pub fn get_file_list(&self, wxid: &str, limit: Option<i64>) -> Result<Vec<MediaInfo>> {
        self.get_contact_media_list(wxid, Some(49), limit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use rusqlite::Connection;

    fn create_test_db() -> (TempDir, String) {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test_media.db");
        let db_path_str = db_path.to_str().unwrap().to_string();
        
        let conn = Connection::open(&db_path).unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS Media (
                MsgSvrId INTEGER PRIMARY KEY,
                BigHeadImgUrl TEXT,
                SmallHeadImgUrl TEXT,
                Reserved0 INTEGER,
                Reserved1 INTEGER,
                Reserved2 INTEGER
            )",
            [],
        ).unwrap();
        
        conn.execute(
            "INSERT INTO Media (MsgSvrId, BigHeadImgUrl, Reserved1, Reserved2) 
             VALUES (?, ?, ?, ?)",
            rusqlite::params![12345, "http://example.com/image.jpg", 3, 1234567890],
        ).unwrap();
        
        (temp_dir, db_path_str)
    }

    #[test]
    fn test_media_handler_new() {
        let (_temp_dir, db_path) = create_test_db();
        let handler = MediaHandler::new(&db_path);
        assert!(handler.is_ok());
    }

    #[test]
    fn test_get_media_info() {
        let (_temp_dir, db_path) = create_test_db();
        let handler = MediaHandler::new(&db_path).unwrap();
        let media = handler.get_media_info(12345).unwrap();
        assert!(media.is_some());
        assert_eq!(media.unwrap().msg_id, 12345);
    }

    #[test]
    fn test_get_image_list() {
        let (_temp_dir, db_path) = create_test_db();
        let handler = MediaHandler::new(&db_path).unwrap();
        let images = handler.get_image_list("test_wxid", Some(10)).unwrap();
        assert!(!images.is_empty());
    }
}

