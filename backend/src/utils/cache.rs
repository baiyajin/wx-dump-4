use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};

/// 缓存项
struct CacheItem<T> {
    value: T,
    expires_at: SystemTime,
}

/// 简单的内存缓存
pub struct Cache<K, V> {
    data: Arc<RwLock<HashMap<K, CacheItem<V>>>>,
    default_ttl: Duration,
}

impl<K, V> Cache<K, V>
where
    K: Hash + Eq + Clone,
    V: Clone,
{
    pub fn new(default_ttl_secs: u64) -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
            default_ttl: Duration::from_secs(default_ttl_secs),
        }
    }

    /// 获取缓存值
    pub fn get(&self, key: &K) -> Option<V> {
        let data = self.data.read().ok()?;
        let item = data.get(key)?;
        
        // 检查是否过期
        if SystemTime::now() > item.expires_at {
            drop(data);
            self.remove(key);
            return None;
        }
        
        Some(item.value.clone())
    }

    /// 设置缓存值
    pub fn set(&self, key: K, value: V) {
        self.set_with_ttl(key, value, self.default_ttl);
    }

    /// 设置缓存值（自定义TTL）
    pub fn set_with_ttl(&self, key: K, value: V, ttl: Duration) {
        let expires_at = SystemTime::now() + ttl;
        let item = CacheItem { value, expires_at };
        
        if let Ok(mut data) = self.data.write() {
            data.insert(key, item);
        }
    }

    /// 删除缓存
    pub fn remove(&self, key: &K) {
        if let Ok(mut data) = self.data.write() {
            data.remove(key);
        }
    }

    /// 清空缓存
    pub fn clear(&self) {
        if let Ok(mut data) = self.data.write() {
            data.clear();
        }
    }

    /// 清理过期项
    pub fn cleanup(&self) {
        let now = SystemTime::now();
        if let Ok(mut data) = self.data.write() {
            data.retain(|_, item| item.expires_at > now);
        }
    }
}

/// 全局缓存管理器
pub struct CacheManager {
    pub msg_count: Cache<String, HashMap<String, i64>>,
    pub date_stats: Cache<String, HashMap<String, serde_json::Value>>,
}

impl CacheManager {
    pub fn new() -> Self {
        Self {
            // 消息数量缓存5分钟
            msg_count: Cache::new(300),
            // 日期统计缓存5分钟
            date_stats: Cache::new(300),
        }
    }

    /// 生成缓存键
    pub fn msg_count_key(db_path: &str, wxid: Option<&str>) -> String {
        format!("msg_count:{}:{}", db_path, wxid.unwrap_or("all"))
    }

    pub fn date_stats_key(db_path: &str, wxid: Option<&str>, start: Option<i64>, end: Option<i64>) -> String {
        format!("date_stats:{}:{}:{}:{}", 
            db_path,
            wxid.unwrap_or("all"),
            start.unwrap_or(0),
            end.unwrap_or(0))
    }
}

impl Default for CacheManager {
    fn default() -> Self {
        Self::new()
    }
}

