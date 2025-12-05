use crate::utils::Result;
use anyhow::Context;
use rusqlite::Connection;
use std::path::Path;

pub struct DatabasePool {
    db_path: String,
}

impl DatabasePool {
    pub fn new(db_path: &str) -> Result<Self> {
        if !Path::new(db_path).exists() {
            return Err(anyhow::anyhow!("Database file not found: {}", db_path).into());
        }

        Ok(Self {
            db_path: db_path.to_string(),
        })
    }

    pub fn get_connection(&self) -> Result<Connection> {
        Connection::open(&self.db_path)
            .with_context(|| format!("Failed to open database: {}", self.db_path))
    }
}

pub struct DatabaseBase {
    pool: DatabasePool,
    existed_tables: Vec<String>,
}

impl DatabaseBase {
    pub fn new(db_path: &str) -> Result<Self> {
        let pool = DatabasePool::new(db_path)?;
        let mut db = Self {
            pool,
            existed_tables: Vec::new(),
        };
        db.load_tables()?;
        Ok(db)
    }

    fn load_tables(&mut self) -> Result<()> {
        let conn = self.pool.get_connection()?;
        let mut stmt = conn.prepare(
            "SELECT name FROM sqlite_master WHERE type='table' AND name!='sqlite_sequence'"
        )?;
        
        let tables = stmt.query_map([], |row| {
            Ok(row.get::<_, String>(0)?)
        })?;

        self.existed_tables.clear();
        for table in tables {
            self.existed_tables.push(table?);
        }

        Ok(())
    }

    pub fn table_exists(&self, table_name: &str) -> bool {
        self.existed_tables.iter().any(|t| t.eq_ignore_ascii_case(table_name))
    }

    pub fn execute_query<T, F>(&self, sql: &str, params: &[&dyn rusqlite::ToSql], mapper: F) -> Result<Vec<T>>
    where
        F: FnMut(&Row) -> rusqlite::Result<T>,
    {
        let conn = self.pool.get_connection()?;
        let mut stmt = conn.prepare(sql)?;
        let rows = stmt.query_map(params, mapper)?;
        
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn execute(&self, sql: &str, params: &[&dyn rusqlite::ToSql]) -> Result<usize> {
        let conn = self.pool.get_connection()?;
        let count = conn.execute(sql, params)?;
        Ok(count)
    }

    pub fn execute_batch(&self, sql: &str) -> Result<()> {
        let conn = self.pool.get_connection()?;
        conn.execute_batch(sql)?;
        Ok(())
    }
}

