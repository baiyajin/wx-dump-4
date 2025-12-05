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

