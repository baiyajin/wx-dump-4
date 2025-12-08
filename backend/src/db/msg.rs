use crate::db::dbbase::DatabaseBase;
use crate::db::msg_query::MsgQuery;
use crate::db::msg_list::MsgList;
use crate::utils::Result;

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

        self.db.execute_batch(
            "CREATE INDEX IF NOT EXISTS idx_MSG_StrTalker ON MSG(StrTalker);
             CREATE INDEX IF NOT EXISTS idx_MSG_CreateTime ON MSG(CreateTime);
             CREATE INDEX IF NOT EXISTS idx_MSG_StrTalker_CreateTime ON MSG(StrTalker, CreateTime);"
        )?;

        Ok(())
    }

    /// 获取消息数量
    pub fn get_msg_count(&self, wxid: Option<&str>) -> Result<std::collections::HashMap<String, i64>> {
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
    ) -> Result<(Vec<crate::db::utils::Message>, Vec<String>)> {
        self.list.get_msg_list_with_users(wxid, start_index, page_size, start_time, end_time)
    }

    /// 获取日期聊天统计
    pub fn get_date_count(
        &self,
        wxid: Option<&str>,
        start_time: Option<i64>,
        end_time: Option<i64>,
    ) -> Result<std::collections::HashMap<String, serde_json::Value>> {
        self.query.get_date_count(wxid, start_time, end_time)
    }

    /// 搜索消息
    pub fn search_messages(
        &self,
        wxid: Option<&str>,
        keyword: &str,
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: i64,
    ) -> Result<Vec<crate::db::utils::Message>> {
        self.list.search_messages(wxid, keyword, start_time, end_time, limit)
    }

    /// 获取聊天最多的联系人
    pub fn get_top_talkers(
        &self,
        top: i64,
        start_time: Option<i64>,
        end_time: Option<i64>,
    ) -> Result<std::collections::HashMap<String, serde_json::Value>> {
        self.query.get_top_talkers(top, start_time, end_time)
    }
}

