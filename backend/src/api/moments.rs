mod models;
mod handlers;

use axum::Router;
use axum::routing::post;
use handlers::*;

pub fn router() -> Router {
    Router::new()
        .route("/api/moments/list", post(get_moments_list))
        .route("/api/moments/count", post(get_moments_count))
}

