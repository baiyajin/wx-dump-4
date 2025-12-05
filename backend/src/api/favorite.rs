mod models;
mod handlers;

use axum::Router;
use axum::routing::post;
use handlers::*;

pub fn router() -> Router {
    Router::new()
        .route("/api/favorite/list", post(get_favorite_list))
        .route("/api/favorite/count", post(get_favorite_count))
}

