use crate::db::dbbase::DatabaseBase;
use crate::db::msg_query::MsgQuery;
use crate::db::msg_list::MsgList;
use crate::db::utils::Message;
use crate::utils::Result;
use std::collections::HashMap;

/// MSG数据库处理器
pub struct MsgHandler {
    db_path: String,
    db: DatabaseBase,
    query: MsgQuery,
    list: MsgList,
}

impl MsgHandler {
    pub fn new(db_path: &str) -> Result<Self> {
        let db = DatabaseBase::new(db_path)?;
        let db_path_str = db_path.to_string();
        let query = MsgQuery::new(DatabaseBase::new(&db_path_str)?);
        let list = MsgList::new(DatabaseBase::new(&db_path_str)?);
        Ok(Self {
            db_path: db_path_str,
            db,
            query,
            list,
        })
    }

    /// 添加索引以加快查询速度
    pub fn add_indexes(&self) -> Result<()> {
        if !self.db.table_exists("MSG") {
            return Ok(());
        }

        // 添加更多索引以优化查询性能
        self.db.execute_batch(
            "CREATE INDEX IF NOT EXISTS idx_MSG_StrTalker ON MSG(StrTalker);
             CREATE INDEX IF NOT EXISTS idx_MSG_CreateTime ON MSG(CreateTime);
             CREATE INDEX IF NOT EXISTS idx_MSG_StrTalker_CreateTime ON MSG(StrTalker, CreateTime);
             CREATE INDEX IF NOT EXISTS idx_MSG_MsgSvrID ON MSG(MsgSvrID);
             CREATE INDEX IF NOT EXISTS idx_MSG_IsSender ON MSG(IsSender);
             CREATE INDEX IF NOT EXISTS idx_MSG_Type ON MSG(Type);
             CREATE INDEX IF NOT EXISTS idx_MSG_StrTalker_IsSender ON MSG(StrTalker, IsSender);"
        )?;

        Ok(())
    }

    /// 获取消息数量
    pub fn get_msg_count(&self, wxid: Option<&str>) -> Result<HashMap<String, i64>> {
        self.query.get_msg_count(wxid)
    }

    /// 获取消息列表（带用户信息）
    pub fn get_msg_list_with_users(
        &self,
        wxid: Option<&str>,
        start_index: i64,
        page_size: i64,
        start_time: Option<i64>,
        end_time: Option<i64>,
    ) -> Result<(Vec<Message>, Vec<String>)> {
        self.list.get_msg_list_with_users(wxid, start_index, page_size, start_time, end_time)
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
        self.list.get_msg_list(wxid, start_index, page_size, start_time, end_time)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use rusqlite::Connection;

    fn create_test_db() -> (TempDir, String) {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test_msg.db");
        let db_path_str = db_path.to_str().unwrap().to_string();
        
        // 创建测试数据库
        let conn = Connection::open(&db_path).unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS MSG (
                localId INTEGER PRIMARY KEY,
                StrTalker TEXT,
                CreateTime INTEGER,
                Type INTEGER,
                SubType INTEGER,
                StrContent TEXT,
                CompressContent BLOB,
                BytesExtra BLOB,
                IsSender INTEGER
            )",
            [],
        ).unwrap();
        
        // 插入测试数据
        conn.execute(
            "INSERT INTO MSG (StrTalker, CreateTime, Type, SubType, StrContent, IsSender) 
             VALUES (?, ?, ?, ?, ?, ?)",
            rusqlite::params!["test_wxid", 1234567890, 1, 0, "Test message", 1],
        ).unwrap();
        
        (temp_dir, db_path_str)
    }

    #[test]
    fn test_msg_handler_new() {
        let (_temp_dir, db_path) = create_test_db();
        let handler = MsgHandler::new(&db_path);
        assert!(handler.is_ok());
    }

    #[test]
    fn test_get_msg_count() {
        let (_temp_dir, db_path) = create_test_db();
        let handler = MsgHandler::new(&db_path).unwrap();
        handler.add_indexes().unwrap();
        
        let counts = handler.get_msg_count(None).unwrap();
        assert!(counts.contains_key("test_wxid"));
        assert_eq!(counts["test_wxid"], 1);
    }

    #[test]
    fn test_get_msg_list() {
        let (_temp_dir, db_path) = create_test_db();
        let handler = MsgHandler::new(&db_path).unwrap();
        handler.add_indexes().unwrap();
        
        let messages = handler.get_msg_list(None, 0, 10, None, None).unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].str_talker, "test_wxid");
    }
}
