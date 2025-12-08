mod models;
mod handlers;

use axum::Router;
use axum::routing::{get, post};
use handlers::*;

pub fn router() -> Router {
    Router::new()
        .route("/api/media/info/:msg_id", post(get_media_info))
        .route("/api/media/list", post(get_media_list))
        .route("/api/media/images", post(get_image_list))
        .route("/api/media/videos", post(get_video_list))
        .route("/api/media/files", post(get_file_list))
        .route("/api/media/img/:msg_id", post(get_image_file))
        .route("/api/media/video/:msg_id", post(get_video_file))
        .route("/api/media/audio/:msg_id", post(get_audio_file))
        .route("/api/media/file/:msg_id", post(get_file_content))
        .route("/api/media/list/:wxid", post(get_contact_media_list))
}

