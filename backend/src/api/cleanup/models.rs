use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzeRequest {
    pub merge_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzeResponse {
    pub total_messages: i64,
    pub db_size: u64,
    pub db_size_mb: f64,
    pub table_count: i64,
    pub index_count: i64,
    pub tables: Vec<TableInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableInfo {
    pub name: String,
    pub row_count: i64,
    pub size_kb: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizeRequest {
    pub merge_path: String,
    pub delete_old_messages: bool,
    pub days_to_keep: Option<i64>,
    pub optimize_database: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizeResponse {
    pub success: bool,
    pub message: String,
    pub deleted_messages: Option<i64>,
    pub optimized: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VacuumRequest {
    pub merge_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VacuumResponse {
    pub success: bool,
    pub message: String,
    pub size_before: u64,
    pub size_after: u64,
    pub size_reduced_mb: f64,
}

