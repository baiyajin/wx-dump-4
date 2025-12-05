mod models;
mod handlers;

use axum::Router;
use axum::routing::{get, post};
use handlers::*;

pub fn router() -> Router {
    Router::new()
        .route("/api/media/info/:msg_id", get(get_media_info))
        .route("/api/media/list", post(get_media_list))
        .route("/api/media/images", post(get_image_list))
        .route("/api/media/videos", post(get_video_list))
        .route("/api/media/files", post(get_file_list))
}

