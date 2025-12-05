use axum::Json;
use std::path::PathBuf;

use crate::db::sns::SnsHandler;
use crate::utils::{AppError, Result};
use super::models::*;

pub async fn get_moments_list(Json(req): Json<MomentsListRequest>) -> Result<Json<MomentsListResponse>> {
    let db_path = PathBuf::from(&req.merge_path);
    if !db_path.exists() {
        return Err(AppError::NotFound(format!("Database not found: {}", req.merge_path)).into());
    }

    let handler = SnsHandler::new(db_path.to_str().unwrap())?;
    let moments = handler.get_moments_list(req.start, req.limit, req.start_time, req.end_time)?;
    let total = handler.get_moments_count()?;

    let response_items: Vec<MomentItemResponse> = moments.into_iter().map(|moment| {
        MomentItemResponse {
            feed_id: moment.feed_id,
            create_time: moment.create_time,
            create_time_str: moment.create_time_str,
            user_name: moment.user_name,
            content: moment.content,
            feed_type: moment.feed_type,
        }
    }).collect();

    Ok(Json(MomentsListResponse {
        moments: response_items,
        total,
    }))
}

pub async fn get_moments_count(Json(req): Json<MomentsCountRequest>) -> Result<Json<MomentsCountResponse>> {
    let db_path = PathBuf::from(&req.merge_path);
    if !db_path.exists() {
        return Err(AppError::NotFound(format!("Database not found: {}", req.merge_path)).into());
    }

    let handler = SnsHandler::new(db_path.to_str().unwrap())?;
    let count = handler.get_moments_count()?;

    Ok(Json(MomentsCountResponse { count }))
}

