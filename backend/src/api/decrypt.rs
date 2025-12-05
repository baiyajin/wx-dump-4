use axum::{Json, Router, routing::post};
use std::path::PathBuf;

use crate::core::decryption::decrypt_db;
use crate::models::wx::{DecryptRequest, DecryptResponse};
use crate::utils::Result;

pub fn router() -> Router {
    Router::new().route("/api/wx/decrypt", post(decrypt_handler))
}

async fn decrypt_handler(Json(req): Json<DecryptRequest>) -> Result<Json<DecryptResponse>> {
    let db_path = PathBuf::from(&req.db_path);
    let out_path = PathBuf::from(&req.out_path);

    match decrypt_db(&req.key, &db_path, &out_path) {
        Ok(_) => Ok(Json(DecryptResponse {
            success: true,
            message: "Decryption successful".to_string(),
            out_path: Some(req.out_path),
        })),
        Err(e) => Ok(Json(DecryptResponse {
            success: false,
            message: format!("Decryption failed: {}", e),
            out_path: None,
        })),
    }
}

