mod models;
mod handlers;

use axum::Router;
use axum::routing::{get, post};
use handlers::*;

pub fn router() -> Router {
    Router::new()
        .route("/api/chat/contacts", post(get_contacts))
        .route("/api/chat/contacts/:wxid", get(get_contact_detail))
        .route("/api/chat/msg/count", post(get_msg_count))
        .route("/api/chat/msg/list", post(get_msg_list))
        .route("/api/chat/msg/search", post(search_messages))
}

