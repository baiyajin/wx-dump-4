use crate::db::dbbase::DatabaseBase;
use crate::utils::Result;
use anyhow::Context;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub wxid: String,
    pub nickname: Option<String>,
    pub remark: Option<String>,
    pub account: Option<String>,
    pub alias: Option<String>,
    pub head_img_url: Option<String>,
    pub contact_type: i32,
}

pub struct ContactHandler {
    db: DatabaseBase,
}

impl ContactHandler {
    pub fn new(db_path: &str) -> Result<Self> {
        let db = DatabaseBase::new(db_path)?;
        Ok(Self { db })
    }

    /// 获取联系人列表
    pub fn get_contacts(&self) -> Result<Vec<Contact>> {
        if !self.db.table_exists("Contact") {
            return Ok(Vec::new());
        }

        let sql = "SELECT UserName, NickName, Remark, Alias, HeadImgUrl, Type 
                   FROM Contact 
                   WHERE UserName IS NOT NULL 
                   ORDER BY NickName";

        let contacts = self.db.execute_query(sql, &[], |row| {
            Ok(Contact {
                wxid: row.get(0)?,
                nickname: row.get(1)?,
                remark: row.get(2)?,
                alias: row.get(3)?,
                head_img_url: row.get(4)?,
                contact_type: row.get(5)?,
            })
        })?;

        Ok(contacts)
    }

    /// 获取联系人详情
    pub fn get_contact(&self, wxid: &str) -> Result<Option<Contact>> {
        if !self.db.table_exists("Contact") {
            return Ok(None);
        }

        let sql = "SELECT UserName, NickName, Remark, Alias, HeadImgUrl, Type 
                   FROM Contact 
                   WHERE UserName = ?";

        let contacts = self.db.execute_query(sql, &[&wxid], |row| {
            Ok(Contact {
                wxid: row.get(0)?,
                nickname: row.get(1)?,
                remark: row.get(2)?,
                alias: row.get(3)?,
                head_img_url: row.get(4)?,
                contact_type: row.get(5)?,
            })
        })?;

        Ok(contacts.first().cloned())
    }

    /// 搜索联系人
    pub fn search_contacts(&self, keyword: &str) -> Result<Vec<Contact>> {
        if !self.db.table_exists("Contact") {
            return Ok(Vec::new());
        }

        let search_pattern = format!("%{}%", keyword);
        let sql = "SELECT UserName, NickName, Remark, Alias, HeadImgUrl, Type 
                   FROM Contact 
                   WHERE UserName LIKE ? OR NickName LIKE ? OR Remark LIKE ? OR Alias LIKE ?
                   ORDER BY NickName";

        let contacts = self.db.execute_query(sql, &[&search_pattern, &search_pattern, &search_pattern, &search_pattern], |row| {
            Ok(Contact {
                wxid: row.get(0)?,
                nickname: row.get(1)?,
                remark: row.get(2)?,
                alias: row.get(3)?,
                head_img_url: row.get(4)?,
                contact_type: row.get(5)?,
            })
        })?;

        Ok(contacts)
    }

    /// 获取联系人映射（用于消息显示）
    pub fn get_contact_map(&self) -> Result<HashMap<String, Contact>> {
        let contacts = self.get_contacts()?;
        let mut map = HashMap::new();
        for contact in contacts {
            map.insert(contact.wxid.clone(), contact);
        }
        Ok(map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use rusqlite::Connection;

    fn create_test_db() -> (TempDir, String) {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test_contact.db");
        let db_path_str = db_path.to_str().unwrap().to_string();
        
        let conn = Connection::open(&db_path).unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS Contact (
                UserName TEXT PRIMARY KEY,
                NickName TEXT,
                Remark TEXT,
                Alias TEXT,
                HeadImgUrl TEXT,
                Type INTEGER
            )",
            [],
        ).unwrap();
        
        conn.execute(
            "INSERT INTO Contact (UserName, NickName, Remark, Type) 
             VALUES (?, ?, ?, ?)",
            rusqlite::params!["test_wxid", "Test Nickname", "Test Remark", 1],
        ).unwrap();
        
        (temp_dir, db_path_str)
    }

    #[test]
    fn test_contact_handler_new() {
        let (_temp_dir, db_path) = create_test_db();
        let handler = ContactHandler::new(&db_path);
        assert!(handler.is_ok());
    }

    #[test]
    fn test_get_contacts() {
        let (_temp_dir, db_path) = create_test_db();
        let handler = ContactHandler::new(&db_path).unwrap();
        let contacts = handler.get_contacts().unwrap();
        assert_eq!(contacts.len(), 1);
        assert_eq!(contacts[0].wxid, "test_wxid");
    }

    #[test]
    fn test_get_contact() {
        let (_temp_dir, db_path) = create_test_db();
        let handler = ContactHandler::new(&db_path).unwrap();
        let contact = handler.get_contact("test_wxid").unwrap();
        assert!(contact.is_some());
        assert_eq!(contact.unwrap().wxid, "test_wxid");
    }

    #[test]
    fn test_search_contacts() {
        let (_temp_dir, db_path) = create_test_db();
        let handler = ContactHandler::new(&db_path).unwrap();
        let contacts = handler.search_contacts("Test").unwrap();
        assert!(!contacts.is_empty());
    }
}

