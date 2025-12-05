mod models;
mod handlers;

use axum::Router;
use axum::routing::post;
use handlers::*;

pub fn router() -> Router {
    Router::new()
        .route("/api/cleanup/analyze", post(analyze_storage))
        .route("/api/cleanup/optimize", post(optimize_database))
        .route("/api/cleanup/vacuum", post(vacuum_database))
}

