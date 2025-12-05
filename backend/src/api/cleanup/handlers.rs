use axum::Json;
use std::path::PathBuf;
use std::fs;

use crate::db::dbbase::DatabaseBase;
use crate::utils::{AppError, Result, validation};
use super::models::*;

pub async fn analyze_storage(Json(req): Json<AnalyzeRequest>) -> Result<Json<AnalyzeResponse>> {
    validation::validate_db_path(&req.merge_path)?;
    
    let db_path = PathBuf::from(&req.merge_path);
    let db = DatabaseBase::new(db_path.to_str().unwrap())?;
    
    // 获取数据库文件大小
    let db_size = fs::metadata(&db_path)?.len();
    let db_size_mb = db_size as f64 / 1024.0 / 1024.0;
    
    // 获取表信息
    let tables = db.execute_query(
        "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'",
        &[],
        |row| Ok(row.get::<_, String>(0)?)
    )?;
    
    let table_count = tables.len() as i64;
    
    // 获取索引数量
    let indexes = db.execute_query(
        "SELECT COUNT(*) FROM sqlite_master WHERE type='index' AND name NOT LIKE 'sqlite_%'",
        &[],
        |row| Ok(row.get::<_, i64>(0)?)
    )?;
    let index_count = indexes.first().copied().unwrap_or(0);
    
    // 获取每个表的信息
    let mut table_infos = Vec::new();
    let mut total_messages = 0i64;
    
    for table_name in &tables {
        if let Ok(rows) = db.execute_query(
            &format!("SELECT COUNT(*) FROM {}", table_name),
            &[],
            |row| Ok(row.get::<_, i64>(0)?)
        ) {
            if let Some(row_count) = rows.first() {
                if table_name == "MSG" {
                    total_messages = *row_count;
                }
                
                // 估算表大小（简化计算）
                let size_kb = (*row_count as f64) * 0.5; // 假设每条记录平均0.5KB
                
                table_infos.push(TableInfo {
                    name: table_name.clone(),
                    row_count: *row_count,
                    size_kb,
                });
            }
        }
    }
    
    Ok(Json(AnalyzeResponse {
        total_messages,
        db_size,
        db_size_mb,
        table_count,
        index_count,
        tables: table_infos,
    }))
}

pub async fn optimize_database(Json(req): Json<OptimizeRequest>) -> Result<Json<OptimizeResponse>> {
    validation::validate_db_path(&req.merge_path)?;
    
    let db_path = PathBuf::from(&req.merge_path);
    let db = DatabaseBase::new(db_path.to_str().unwrap())?;
    
    let mut deleted_messages = None;
    
    // 删除旧消息
    if req.delete_old_messages {
        if let Some(days) = req.days_to_keep {
            let cutoff_time = chrono::Local::now().timestamp() - (days * 24 * 60 * 60);
            
            if db.table_exists("MSG") {
                let deleted = db.execute(
                    "DELETE FROM MSG WHERE CreateTime < ?",
                    &[&cutoff_time]
                )?;
                deleted_messages = Some(deleted as i64);
            }
        }
    }
    
    // 优化数据库
    let optimized = if req.optimize_database {
        db.execute_batch("VACUUM; ANALYZE;")?;
        true
    } else {
        false
    };
    
    Ok(Json(OptimizeResponse {
        success: true,
        message: format!(
            "优化完成。{} {}",
            if let Some(count) = deleted_messages {
                format!("删除了 {} 条旧消息。", count)
            } else {
                String::new()
            },
            if optimized { "数据库已优化。" } else { "" }
        ),
        deleted_messages,
        optimized,
    }))
}

pub async fn vacuum_database(Json(req): Json<VacuumRequest>) -> Result<Json<VacuumResponse>> {
    validation::validate_db_path(&req.merge_path)?;
    
    let db_path = PathBuf::from(&req.merge_path);
    let size_before = fs::metadata(&db_path)?.len();
    
    let db = DatabaseBase::new(db_path.to_str().unwrap())?;
    db.execute_batch("VACUUM;")?;
    
    let size_after = fs::metadata(&db_path)?.len();
    let size_reduced_mb = (size_before.saturating_sub(size_after)) as f64 / 1024.0 / 1024.0;
    
    Ok(Json(VacuumResponse {
        success: true,
        message: format!(
            "数据库优化完成。释放了 {:.2} MB 空间。",
            size_reduced_mb
        ),
        size_before,
        size_after,
        size_reduced_mb,
    }))
}

