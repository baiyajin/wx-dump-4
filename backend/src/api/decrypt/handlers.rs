use axum::Json;
use std::path::PathBuf;

use crate::core::decryption::decrypt_db;
use crate::utils::{AppError, Result, validation};
use super::models::*;

pub async fn decrypt_db_handler(Json(req): Json<DecryptRequest>) -> Result<Json<DecryptResponse>> {
    validation::validate_db_path(&req.db_path)?;
    validation::validate_key(&req.key)?;
    
    let db_path = PathBuf::from(&req.db_path);
    let output_path = req.output_path.unwrap_or_else(|| {
        let mut path = db_path.clone();
        path.set_file_name(format!(
            "{}_decrypted.db",
            path.file_stem().and_then(|s| s.to_str()).unwrap_or("db")
        ));
        path.to_string_lossy().to_string()
    });

    match decrypt_db(&req.key, &db_path, &PathBuf::from(&output_path)) {
        Ok(_) => Ok(Json(DecryptResponse {
            success: true,
            message: format!("Database decrypted successfully to: {}", output_path),
            decrypted_path: Some(output_path),
        })),
        Err(e) => Ok(Json(DecryptResponse {
            success: false,
            message: format!("Decryption failed: {}", e),
            decrypted_path: None,
        })),
    }
}

