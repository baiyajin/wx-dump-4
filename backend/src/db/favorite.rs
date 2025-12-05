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

