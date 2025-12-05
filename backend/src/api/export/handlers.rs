use axum::Json;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::db::msg::MsgHandler;
use crate::utils::{AppError, Result};
use super::models::*;
use super::csv_export::CsvExporter;
use super::json_export::JsonExporter;
use super::html_export::HtmlExporter;

pub async fn export_csv(Json(req): Json<ExportRequest>) -> Result<Json<ExportResponse>> {
    let db_path = PathBuf::from(&req.merge_path);
    if !db_path.exists() {
        return Ok(Json(ExportResponse {
            success: false,
            message: format!("Database not found: {}", req.merge_path),
            file_path: None,
        }));
    }

    let handler = MsgHandler::new(db_path.to_str().unwrap())?;
    handler.add_indexes()?;

    let output_path = req.output_path.unwrap_or_else(|| {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        format!("export_{}.csv", timestamp)
    });

    match CsvExporter::export(
        &handler,
        req.wxid.as_deref(),
        req.start_time,
        req.end_time,
        &output_path,
    ) {
        Ok(message) => Ok(Json(ExportResponse {
            success: true,
            message,
            file_path: Some(output_path),
        })),
        Err(e) => Ok(Json(ExportResponse {
            success: false,
            message: format!("导出失败: {}", e),
            file_path: None,
        })),
    }
}

pub async fn export_json(Json(req): Json<ExportRequest>) -> Result<Json<ExportResponse>> {
    let db_path = PathBuf::from(&req.merge_path);
    if !db_path.exists() {
        return Ok(Json(ExportResponse {
            success: false,
            message: format!("Database not found: {}", req.merge_path),
            file_path: None,
        }));
    }

    let handler = MsgHandler::new(db_path.to_str().unwrap())?;
    handler.add_indexes()?;

    let output_path = req.output_path.unwrap_or_else(|| {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        format!("export_{}.json", timestamp)
    });

    match JsonExporter::export(
        &handler,
        req.wxid.as_deref(),
        req.start_time,
        req.end_time,
        &output_path,
    ) {
        Ok(message) => Ok(Json(ExportResponse {
            success: true,
            message,
            file_path: Some(output_path),
        })),
        Err(e) => Ok(Json(ExportResponse {
            success: false,
            message: format!("导出失败: {}", e),
            file_path: None,
        })),
    }
}

pub async fn export_html(Json(req): Json<ExportRequest>) -> Result<Json<ExportResponse>> {
    let db_path = PathBuf::from(&req.merge_path);
    if !db_path.exists() {
        return Ok(Json(ExportResponse {
            success: false,
            message: format!("Database not found: {}", req.merge_path),
            file_path: None,
        }));
    }

    let handler = MsgHandler::new(db_path.to_str().unwrap())?;
    handler.add_indexes()?;

    let output_path = req.output_path.unwrap_or_else(|| {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        format!("export_{}.html", timestamp)
    });

    match HtmlExporter::export(
        &handler,
        req.wxid.as_deref(),
        req.start_time,
        req.end_time,
        &output_path,
    ) {
        Ok(message) => Ok(Json(ExportResponse {
            success: true,
            message,
            file_path: Some(output_path),
        })),
        Err(e) => Ok(Json(ExportResponse {
            success: false,
            message: format!("导出失败: {}", e),
            file_path: None,
        })),
    }
}

