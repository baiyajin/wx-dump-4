mod models;
mod handlers;

use axum::Router;
use axum::routing::post;
use handlers::*;

pub fn router() -> Router {
    Router::new()
        .route("/api/merge/databases", post(merge_databases))
}

