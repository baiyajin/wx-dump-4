use axum::{
    extract::DefaultBodyLimit,
    http::Method,
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    limit::RequestBodyLimitLayer,
};
use tracing::{info, Level};
use tracing_subscriber;

mod api;
mod config;
mod core;
mod db;
mod models;
mod utils;

use api::create_router;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_target(false)
        .init();

    info!("Starting wx-dump-4 backend server...");

    // 加载配置
    let wx_offs = config::load_wx_offs()?;
    info!("Loaded {} WeChat version offsets", wx_offs.len());

    // 创建路由
    let app = create_router(wx_offs);

    // 配置CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(Any);

    // 构建应用
    let app = app
        .layer(
            ServiceBuilder::new()
                .layer(cors)
                .layer(DefaultBodyLimit::disable())
                .layer(RequestBodyLimitLayer::new(100 * 1024 * 1024)), // 100MB
        );

    // 启动服务器
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

