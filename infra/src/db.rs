//! 持久化：连接池、迁移与数据表仓储。

pub mod game;
pub mod member;
pub mod user;

use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

use crate::{config::DatabaseConfig, error::AppError};

/// 连接数据库并执行迁移
pub async fn connect(config: &DatabaseConfig) -> Result<SqlitePool, AppError> {
    // 1) 建立连接池；连接数至少为 1，避免 0 配置导致连接池不可用
    let pool = SqlitePoolOptions::new()
        .max_connections(config.max_connections.max(1))
        .connect(&config.url)
        .await?;

    // 2) 执行 migrations 目录下的全部迁移
    sqlx::migrate!("../migrations").run(&pool).await?;

    // 3) 将迁移中的默认账户明文密码升级为 Argon2 哈希
    user::upgrade_plaintext_passwords(&pool).await?;

    Ok(pool)
}

/// 解析以 JSON 文本存储的字符串数组；损坏数据降级为空列表并记录告警
pub(crate) fn decode_list(raw: &str) -> Vec<String> {
    serde_json::from_str::<Vec<String>>(raw).unwrap_or_else(|error| {
        tracing::warn!(%error, "列表字段 JSON 解析失败，已降级为空列表");
        Vec::new()
    })
}

/// 将以 JSON 文本存储的字符串数组；序列化失败时降级为空数组
pub(crate) fn encode_list(values: &[String]) -> String {
    serde_json::to_string(values).unwrap_or_else(|error| {
        tracing::warn!(%error, "列表字段 JSON 序列化失败，已降级为空数组");
        "[]".to_string()
    })
}
