use axum::Json;
use std::path::PathBuf;

use crate::db::favorite::FavoriteHandler;
use crate::utils::{AppError, Result};
use super::models::*;

pub async fn get_favorite_list(Json(req): Json<FavoriteListRequest>) -> Result<Json<FavoriteListResponse>> {
    let db_path = PathBuf::from(&req.merge_path);
    if !db_path.exists() {
        return Err(AppError::NotFound(format!("Database not found: {}", req.merge_path)).into());
    }

    let handler = FavoriteHandler::new(db_path.to_str().unwrap())?;
    let favorites = handler.get_favorite_list(req.start, req.limit, req.start_time, req.end_time)?;
    let total = handler.get_favorite_count()?;

    let response_items: Vec<FavoriteItemResponse> = favorites.into_iter().map(|fav| {
        FavoriteItemResponse {
            id: fav.id,
            local_id: fav.local_id,
            talker: fav.talker,
            content: fav.content,
            create_time: fav.create_time,
            create_time_str: fav.create_time_str,
            msg_type: fav.msg_type,
            type_name: fav.type_name,
            src: fav.src,
        }
    }).collect();

    Ok(Json(FavoriteListResponse {
        favorites: response_items,
        total,
    }))
}

pub async fn get_favorite_count(Json(req): Json<FavoriteCountRequest>) -> Result<Json<FavoriteCountResponse>> {
    let db_path = PathBuf::from(&req.merge_path);
    if !db_path.exists() {
        return Err(AppError::NotFound(format!("Database not found: {}", req.merge_path)).into());
    }

    let handler = FavoriteHandler::new(db_path.to_str().unwrap())?;
    let count = handler.get_favorite_count()?;

    Ok(Json(FavoriteCountResponse { count }))
}

