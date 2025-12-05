use axum::{Json, extract::Path};
use std::path::PathBuf;

use crate::db::media::MediaHandler;
use crate::utils::{AppError, Result};
use super::models::*;

pub async fn get_media_info(
    Path(msg_id): Path<i64>,
    Json(req): Json<MediaInfoRequest>
) -> Result<Json<MediaInfoResponse>> {
    let db_path = PathBuf::from(&req.media_db_path);
    if !db_path.exists() {
        return Err(AppError::NotFound(format!("Media database not found: {}", req.media_db_path)).into());
    }

    let handler = MediaHandler::new(db_path.to_str().unwrap())?;
    let media_info = handler.get_media_info(msg_id)?;

    if let Some(info) = media_info {
        Ok(Json(MediaInfoResponse {
            msg_id: info.msg_id,
            media_path: info.media_path,
            thumb_path: info.thumb_path,
            file_size: info.file_size,
            media_type: info.media_type,
            create_time: info.create_time,
        }))
    } else {
        Err(AppError::NotFound(format!("Media info not found for msg_id: {}", msg_id)).into())
    }
}

pub async fn get_media_list(
    Json(req): Json<MediaListRequest>
) -> Result<Json<MediaListResponse>> {
    let db_path = PathBuf::from(&req.media_db_path);
    if !db_path.exists() {
        return Err(AppError::NotFound(format!("Media database not found: {}", req.media_db_path)).into());
    }

    let handler = MediaHandler::new(db_path.to_str().unwrap())?;
    let media_list = handler.get_contact_media_list(&req.wxid, req.media_type, req.limit)?;

    let response_list: Vec<MediaInfoResponse> = media_list.into_iter().map(|info| {
        MediaInfoResponse {
            msg_id: info.msg_id,
            media_path: info.media_path,
            thumb_path: info.thumb_path,
            file_size: info.file_size,
            media_type: info.media_type,
            create_time: info.create_time,
        }
    }).collect();

    Ok(Json(MediaListResponse {
        media_list: response_list,
    }))
}

pub async fn get_image_list(
    Json(req): Json<MediaListRequest>
) -> Result<Json<MediaListResponse>> {
    let db_path = PathBuf::from(&req.media_db_path);
    if !db_path.exists() {
        return Err(AppError::NotFound(format!("Media database not found: {}", req.media_db_path)).into());
    }

    let handler = MediaHandler::new(db_path.to_str().unwrap())?;
    let media_list = handler.get_image_list(&req.wxid, req.limit)?;

    let response_list: Vec<MediaInfoResponse> = media_list.into_iter().map(|info| {
        MediaInfoResponse {
            msg_id: info.msg_id,
            media_path: info.media_path,
            thumb_path: info.thumb_path,
            file_size: info.file_size,
            media_type: info.media_type,
            create_time: info.create_time,
        }
    }).collect();

    Ok(Json(MediaListResponse {
        media_list: response_list,
    }))
}

pub async fn get_video_list(
    Json(req): Json<MediaListRequest>
) -> Result<Json<MediaListResponse>> {
    let db_path = PathBuf::from(&req.media_db_path);
    if !db_path.exists() {
        return Err(AppError::NotFound(format!("Media database not found: {}", req.media_db_path)).into());
    }

    let handler = MediaHandler::new(db_path.to_str().unwrap())?;
    let media_list = handler.get_video_list(&req.wxid, req.limit)?;

    let response_list: Vec<MediaInfoResponse> = media_list.into_iter().map(|info| {
        MediaInfoResponse {
            msg_id: info.msg_id,
            media_path: info.media_path,
            thumb_path: info.thumb_path,
            file_size: info.file_size,
            media_type: info.media_type,
            create_time: info.create_time,
        }
    }).collect();

    Ok(Json(MediaListResponse {
        media_list: response_list,
    }))
}

pub async fn get_file_list(
    Json(req): Json<MediaListRequest>
) -> Result<Json<MediaListResponse>> {
    let db_path = PathBuf::from(&req.media_db_path);
    if !db_path.exists() {
        return Err(AppError::NotFound(format!("Media database not found: {}", req.media_db_path)).into());
    }

    let handler = MediaHandler::new(db_path.to_str().unwrap())?;
    let media_list = handler.get_file_list(&req.wxid, req.limit)?;

    let response_list: Vec<MediaInfoResponse> = media_list.into_iter().map(|info| {
        MediaInfoResponse {
            msg_id: info.msg_id,
            media_path: info.media_path,
            thumb_path: info.thumb_path,
            file_size: info.file_size,
            media_type: info.media_type,
            create_time: info.create_time,
        }
    }).collect();

    Ok(Json(MediaListResponse {
        media_list: response_list,
    }))
}

