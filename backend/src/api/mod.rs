use axum::Router;
use std::collections::HashMap;

mod wx_info;
mod decrypt;
mod chat;
mod statistics;

pub fn create_router(wx_offs: HashMap<String, Vec<u32>>) -> Router {
    Router::new()
        .merge(wx_info::router(wx_offs.clone()))
        .merge(decrypt::router())
        .merge(chat::router())
        .merge(statistics::router())
        .route("/health", axum::routing::get(health_check))
}

async fn health_check() -> &'static str {
    "OK"
}

