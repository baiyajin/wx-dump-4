use axum::Json;

use crate::db::merge::DbMerger;
use crate::utils::Result;
use super::models::*;

pub async fn merge_databases(Json(req): Json<MergeRequest>) -> Result<Json<MergeResponse>> {
    let remove_duplicates = req.remove_duplicates.unwrap_or(true);

    match DbMerger::merge_databases(
        &req.source_paths,
        &req.output_path,
        remove_duplicates,
    ) {
        Ok(total_inserted) => Ok(Json(MergeResponse {
            success: true,
            message: format!("成功合并 {} 条消息", total_inserted),
            total_inserted: Some(total_inserted),
        })),
        Err(e) => Ok(Json(MergeResponse {
            success: false,
            message: format!("合并失败: {}", e),
            total_inserted: None,
        })),
    }
}

