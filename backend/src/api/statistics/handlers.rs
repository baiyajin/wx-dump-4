use axum::{Json, extract::Path};
use std::collections::HashMap;
use std::path::PathBuf;
use regex::Regex;

use crate::db::msg::MsgHandler;
use crate::utils::{AppError, Result};
use super::models::*;

pub async fn get_contact_stat(
    Path(wxid): Path<String>,
    Json(req): Json<ContactStatRequest>
) -> Result<Json<ContactStatResponse>> {
    let db_path = PathBuf::from(&req.merge_path);
    if !db_path.exists() {
        return Err(AppError::NotFound(format!("Database not found: {}", req.merge_path)).into());
    }

    let handler = MsgHandler::new(db_path.to_str().unwrap())?;
    handler.add_indexes()?;

    let counts = handler.get_msg_count(Some(&wxid))?;
    let total = counts.get(&wxid).copied().unwrap_or(0);

    // 获取日期统计
    let date_stats = handler.get_date_count(Some(&wxid), req.start_time, req.end_time)?;

    // 计算发送和接收数量
    let mut sender_count = 0;
    let mut receiver_count = 0;
    for (_, stat) in &date_stats {
        if let Some(s) = stat.get("sender_count").and_then(|v| v.as_i64()) {
            sender_count += s;
        }
        if let Some(r) = stat.get("receiver_count").and_then(|v| v.as_i64()) {
            receiver_count += r;
        }
    }

    Ok(Json(ContactStatResponse {
        wxid,
        total_count: total,
        sender_count,
        receiver_count,
        date_stats,
    }))
}

pub async fn get_date_chat_stat(
    Json(req): Json<DateChatStatRequest>
) -> Result<Json<DateChatStatResponse>> {
    let db_path = PathBuf::from(&req.merge_path);
    if !db_path.exists() {
        return Err(AppError::NotFound(format!("Database not found: {}", req.merge_path)).into());
    }

    let handler = MsgHandler::new(db_path.to_str().unwrap())?;
    handler.add_indexes()?;

    let stats = handler.get_date_count(req.wxid.as_deref(), req.start_time, req.end_time)?;

    Ok(Json(DateChatStatResponse { stats }))
}

pub async fn get_date_heatmap(
    Json(req): Json<DateHeatmapRequest>
) -> Result<Json<DateHeatmapResponse>> {
    let db_path = PathBuf::from(&req.merge_path);
    if !db_path.exists() {
        return Err(AppError::NotFound(format!("Database not found: {}", req.merge_path)).into());
    }

    let handler = MsgHandler::new(db_path.to_str().unwrap())?;
    handler.add_indexes()?;

    let stats = handler.get_date_count(req.wxid.as_deref(), req.start_time, req.end_time)?;

    let data: Vec<HeatmapData> = stats
        .into_iter()
        .map(|(date, stat)| {
            let count = stat
                .get("total_count")
                .and_then(|v| v.as_i64())
                .unwrap_or(0);
            HeatmapData { date, count }
        })
        .collect();

    Ok(Json(DateHeatmapResponse { data }))
}

pub async fn get_top_talkers(
    Json(req): Json<TopTalkersRequest>
) -> Result<Json<TopTalkersResponse>> {
    let db_path = PathBuf::from(&req.merge_path);
    if !db_path.exists() {
        return Err(AppError::NotFound(format!("Database not found: {}", req.merge_path)).into());
    }

    let handler = MsgHandler::new(db_path.to_str().unwrap())?;
    handler.add_indexes()?;

    let top = req.top.unwrap_or(10);
    let talkers = handler.get_top_talkers(top, req.start_time, req.end_time)?;

    Ok(Json(TopTalkersResponse { talkers }))
}

pub async fn get_wordcloud(
    Path(wxid): Path<String>,
    Json(req): Json<WordcloudRequest>
) -> Result<Json<WordcloudResponse>> {
    let db_path = PathBuf::from(&req.merge_path);
    if !db_path.exists() {
        return Err(AppError::NotFound(format!("Database not found: {}", req.merge_path)).into());
    }

    let handler = MsgHandler::new(db_path.to_str().unwrap())?;
    handler.add_indexes()?;

    // 获取文本消息内容
    let messages = handler.get_msg_list(
        Some(&wxid),
        req.start_time,
        req.end_time,
        None,
        None,
    )?;

    // 提取文本并统计词频
    let mut word_count: HashMap<String, i64> = HashMap::new();
    
    for msg in messages {
        // 只处理文本消息（msg_type = 1）
        if msg.msg_type == Some(1) {
            if let Some(ref content) = msg.content {
                // 简单的分词：按中文字符、英文单词分割
                let words = extract_words(content);
                for word in words {
                    if word.len() > 1 { // 过滤单字符
                        *word_count.entry(word).or_insert(0) += 1;
                    }
                }
            }
        }
    }

    // 转换为Vec并排序
    let mut word_list: Vec<WordData> = word_count
        .into_iter()
        .map(|(word, count)| WordData { word, count })
        .collect();
    
    word_list.sort_by(|a, b| b.count.cmp(&a.count));
    
    // 限制返回数量
    let limit = req.limit.unwrap_or(100);
    word_list.truncate(limit);

    Ok(Json(WordcloudResponse { words: word_list }))
}

/// 简单的分词函数：提取中文字词和英文单词
fn extract_words(text: &str) -> Vec<String> {
    let mut words = Vec::new();
    
    // 移除标点符号和特殊字符
    let cleaned = text
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace() || (*c as u32 >= 0x4E00 && *c as u32 <= 0x9FFF))
        .collect::<String>();
    
    // 提取中文词汇（2-4字词）
    let chinese_re = Regex::new(r"[\u{4E00}-\u{9FFF}]{2,4}").unwrap();
    for mat in chinese_re.find_iter(&cleaned) {
        words.push(mat.as_str().to_string());
    }
    
    // 提取英文单词
    let english_re = Regex::new(r"\b[a-zA-Z]{2,}\b").unwrap();
    for mat in english_re.find_iter(&cleaned) {
        words.push(mat.as_str().to_lowercase());
    }
    
    words
}

