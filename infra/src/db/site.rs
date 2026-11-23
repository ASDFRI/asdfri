//! 站点设置的持久化。

use domain::SiteVariant;
use sqlx::SqlitePool;

use crate::error::AppError;

/// 读取站点视觉变体；缺失或损坏时回退为默认值
pub async fn get_variant(pool: &SqlitePool) -> Result<SiteVariant, AppError> {
    let raw: Option<String> = sqlx::query_scalar("SELECT variant FROM site_settings WHERE id = 1")
        .fetch_optional(pool)
        .await?;

    // 1) 正常值直接返回
    if let Some(variant) = raw.as_deref().and_then(SiteVariant::parse) {
        return Ok(variant);
    }

    // 2) 缺失或损坏时告警并回退，保证页面始终可渲染
    tracing::warn!("站点视觉变体缺失或无效，已回退为默认值");

    Ok(SiteVariant::default())
}

/// 写入站点视觉变体
pub async fn set_variant(pool: &SqlitePool, variant: SiteVariant) -> Result<(), AppError> {
    sqlx::query(
        "INSERT INTO site_settings (id, variant) VALUES (1, ?) \
         ON CONFLICT(id) DO UPDATE SET variant = excluded.variant",
    )
    .bind(variant.as_str())
    .execute(pool)
    .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use sqlx::sqlite::SqlitePoolOptions;

    use super::*;

    async fn pool() -> SqlitePool {
        let pool =
            SqlitePoolOptions::new().max_connections(1).connect("sqlite::memory:").await.unwrap();
        sqlx::query("CREATE TABLE site_settings (id INTEGER PRIMARY KEY, variant TEXT NOT NULL)")
            .execute(&pool)
            .await
            .unwrap();

        pool
    }

    #[tokio::test]
    async fn persists_variant() {
        let pool = pool().await;

        set_variant(&pool, SiteVariant::Constructivist).await.unwrap();
        assert_eq!(get_variant(&pool).await.unwrap(), SiteVariant::Constructivist);

        set_variant(&pool, SiteVariant::Swiss).await.unwrap();
        assert_eq!(get_variant(&pool).await.unwrap(), SiteVariant::Swiss);
    }

    #[tokio::test]
    async fn falls_back_on_missing_or_invalid_value() {
        let pool = pool().await;
        assert_eq!(get_variant(&pool).await.unwrap(), SiteVariant::Swiss);

        sqlx::query("INSERT INTO site_settings (id, variant) VALUES (1, 'brutalist')")
            .execute(&pool)
            .await
            .unwrap();
        assert_eq!(get_variant(&pool).await.unwrap(), SiteVariant::Swiss);
    }
}
