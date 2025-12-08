mod models;
mod handlers;

use axum::Router;
use axum::routing::post;
use handlers::*;

pub fn router() -> Router {
    Router::new()
        .route("/api/stat/contact/:wxid", post(get_contact_stat))
        .route("/api/stat/date/chat", post(get_date_chat_stat))
        .route("/api/stat/date/heatmap", post(get_date_heatmap))
        .route("/api/stat/top/talkers", post(get_top_talkers))
        .route("/api/stat/wordcloud/:wxid", post(get_wordcloud))
}

