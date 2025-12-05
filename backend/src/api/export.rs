mod models;
mod handlers;
mod csv_export;
mod json_export;
mod html_export;

use axum::Router;
use axum::routing::post;
use handlers::*;

pub fn router() -> Router {
    Router::new()
        .route("/api/export/csv", post(export_csv))
        .route("/api/export/json", post(export_json))
        .route("/api/export/html", post(export_html))
}

