// ... existing code ...

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
}
