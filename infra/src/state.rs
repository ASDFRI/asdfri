use std::sync::{Arc, RwLock};

use axum::extract::FromRef;
use domain::SiteVariant;
use leptos::config::LeptosOptions;
use sqlx::SqlitePool;

use crate::{auth::Keys, config::AppConfig};

/// 站点视觉变体缓存
///
/// 数据库为唯一事实来源；缓存避免每次 SSR 都查询数据库，
/// 写入路径在持久化成功后同步更新。
#[derive(Clone, Debug)]
pub struct SiteTheme(Arc<RwLock<SiteVariant>>);

impl SiteTheme {
    /// 使用初始变体创建缓存
    pub fn new(variant: SiteVariant) -> Self {
        Self(Arc::new(RwLock::new(variant)))
    }

    /// 读取当前变体
    pub fn get(&self) -> SiteVariant {
        *self.0.read().unwrap_or_else(|error| error.into_inner())
    }

    /// 覆盖当前变体
    pub fn set(&self, variant: SiteVariant) {
        *self.0.write().unwrap_or_else(|error| error.into_inner()) = variant;
    }
}

/// 全局应用状态，注入 Axum 与 Leptos 上下文
#[derive(Clone, FromRef)]
pub struct AppState {
    /// 应用配置
    pub config: AppConfig,
    /// JWT 编解码密钥
    pub keys: Keys,
    /// Leptos 运行时选项
    pub leptos_options: LeptosOptions,
    /// 数据库连接池
    pub pool: SqlitePool,
    /// 站点视觉变体缓存
    pub theme: SiteTheme,
}
