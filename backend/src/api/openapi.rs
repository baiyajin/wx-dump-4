use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

/// OpenAPI文档定义
#[derive(OpenApi)]
#[openapi(
    info(
        title = "wx-dump-4 API",
        description = "微信数据导出工具 API 文档",
        version = "0.1.0"
    ),
    tags(
        (name = "微信信息", description = "微信信息获取相关API"),
        (name = "数据库解密", description = "数据库解密相关API"),
        (name = "聊天记录", description = "聊天记录查询相关API"),
        (name = "数据导出", description = "数据导出相关API"),
        (name = "统计分析", description = "统计分析相关API"),
        (name = "媒体文件", description = "媒体文件相关API"),
        (name = "收藏", description = "收藏相关API"),
        (name = "朋友圈", description = "朋友圈相关API"),
        (name = "工具", description = "工具相关API"),
    ),
)]
pub struct ApiDoc;

/// 创建Swagger UI路由
pub fn swagger_router() -> Router {
    SwaggerUi::new("/swagger-ui/{_:.*}")
        .url("/api-doc/openapi.json", ApiDoc::openapi())
        .into()
}
