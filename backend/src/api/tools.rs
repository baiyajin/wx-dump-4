mod models;
mod handlers;

use axum::Router;
use axum::routing::{get, post};
use handlers::*;

pub fn router() -> Router {
    Router::new()
        .route("/api/tools/bias", post(get_bias))
        .route("/api/tools/decrypt", post(decrypt_db))
        .route("/api/tools/merge", post(merge_db))
        .route("/api/tools/wxinfo", get(get_wxinfo))
}

