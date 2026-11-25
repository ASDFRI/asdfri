//! 站点设置的持久化。

use domain::{HeroContent, HeroDraft, SiteVariant};
use sqlx::{FromRow, SqlitePool};

use crate::{
    db::{decode_list, encode_list},
    error::AppError,
};

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

/// `hero_content` 表行映射
#[derive(FromRow)]
struct HeroRow {
    eyebrow: String,
    location: String,
    title: String,
    lede: String,
    primary_label: String,
    primary_href: String,
    secondary_label: String,
    secondary_href: String,
    media_tag: String,
    media_poster: String,
    media_video: String,
    strip: String,
}

impl HeroRow {
    /// 将数据库行转换为内容模型；必填字段损坏时回退为默认值
    fn into_content(self, variant: SiteVariant) -> HeroContent {
        let defaults = HeroContent::defaults(variant);
        let title = clean_lines(&decode_list(&self.title), 3);
        let strip = clean_lines(&decode_list(&self.strip), 8);

        HeroContent {
            eyebrow: required_text(self.eyebrow, defaults.eyebrow),
            location: self.location.trim().to_string(),
            title: if title.is_empty() {
                defaults.title
            } else {
                title
            },
            lede: required_text(self.lede, defaults.lede),
            primary_label: required_text(self.primary_label, defaults.primary_label),
            primary_href: required_text(self.primary_href, defaults.primary_href),
            secondary_label: self.secondary_label.trim().to_string(),
            secondary_href: self.secondary_href.trim().to_string(),
            media_tag: self.media_tag.trim().to_string(),
            media_poster: self.media_poster.trim().to_string(),
            media_video: self.media_video.trim().to_string(),
            strip,
        }
    }
}

/// 读取指定视觉变体的首屏内容；缺失时回退为默认值
pub async fn get_hero(pool: &SqlitePool, variant: SiteVariant) -> Result<HeroContent, AppError> {
    let row = sqlx::query_as::<_, HeroRow>(
        "SELECT eyebrow, location, title, lede, primary_label, primary_href, \
         secondary_label, secondary_href, media_tag, media_poster, media_video, strip \
         FROM hero_content WHERE variant = ?",
    )
    .bind(variant.as_str())
    .fetch_optional(pool)
    .await?;

    Ok(row.map_or_else(|| HeroContent::defaults(variant), |row| row.into_content(variant)))
}

/// 写入指定视觉变体的首屏内容
pub async fn set_hero(
    pool: &SqlitePool,
    variant: SiteVariant,
    draft: &HeroDraft,
) -> Result<(), AppError> {
    sqlx::query(
        "INSERT INTO hero_content (variant, eyebrow, location, title, lede, primary_label, \
         primary_href, secondary_label, secondary_href, media_tag, media_poster, media_video, strip) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?) \
         ON CONFLICT(variant) DO UPDATE SET eyebrow = excluded.eyebrow, \
         location = excluded.location, title = excluded.title, lede = excluded.lede, \
         primary_label = excluded.primary_label, primary_href = excluded.primary_href, \
         secondary_label = excluded.secondary_label, secondary_href = excluded.secondary_href, \
         media_tag = excluded.media_tag, media_poster = excluded.media_poster, \
         media_video = excluded.media_video, strip = excluded.strip",
    )
    .bind(variant.as_str())
    .bind(&draft.eyebrow)
    .bind(&draft.location)
    .bind(encode_list(&draft.title))
    .bind(&draft.lede)
    .bind(&draft.primary_label)
    .bind(&draft.primary_href)
    .bind(&draft.secondary_label)
    .bind(&draft.secondary_href)
    .bind(&draft.media_tag)
    .bind(&draft.media_poster)
    .bind(&draft.media_video)
    .bind(encode_list(&draft.strip))
    .execute(pool)
    .await?;

    Ok(())
}

/// 裁剪必填文本；空值回退到默认值
fn required_text(value: String, fallback: String) -> String {
    let value = value.trim();

    if value.is_empty() {
        fallback
    } else {
        value.to_string()
    }
}

/// 清洗文本列表：裁剪、丢弃空项并限制长度
fn clean_lines(values: &[String], limit: usize) -> Vec<String> {
    values
        .iter()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .take(limit)
        .collect()
}

#[cfg(test)]
mod tests {
    use sqlx::sqlite::SqlitePoolOptions;

    use super::*;

    async fn pool() -> SqlitePool {
        let pool =
            SqlitePoolOptions::new().max_connections(1).connect("sqlite::memory:").await.unwrap();
        sqlx::migrate!("../migrations").run(&pool).await.unwrap();

        pool
    }

    #[tokio::test]
    async fn persists_variant() {
        let pool = pool().await;

        for variant in SiteVariant::ALL {
            set_variant(&pool, variant).await.unwrap();
            assert_eq!(get_variant(&pool).await.unwrap(), variant);
        }
    }

    #[tokio::test]
    async fn falls_back_on_missing_or_invalid_value() {
        let pool = pool().await;
        assert_eq!(get_variant(&pool).await.unwrap(), SiteVariant::Swiss);

        sqlx::query("UPDATE site_settings SET variant = 'brutalist' WHERE id = 1")
            .execute(&pool)
            .await
            .unwrap();
        assert_eq!(get_variant(&pool).await.unwrap(), SiteVariant::Swiss);
    }

    #[tokio::test]
    async fn hero_falls_back_to_variant_defaults() {
        let pool = pool().await;

        for variant in SiteVariant::ALL {
            assert_eq!(get_hero(&pool, variant).await.unwrap(), HeroContent::defaults(variant));
        }
    }

    #[tokio::test]
    async fn persists_hero_content_per_variant() {
        let pool = pool().await;
        let mut expected = Vec::new();

        for (index, variant) in SiteVariant::ALL.into_iter().enumerate() {
            let mut content = HeroContent::defaults(variant);
            content.eyebrow = format!("TEST / {}", variant.as_str());
            content.title = vec![format!("LINE {index}")];
            content.strip = vec![format!("TAG {index}")];

            set_hero(&pool, variant, &HeroDraft::from(content.clone())).await.unwrap();
            expected.push((variant, content));
        }

        for (variant, content) in expected {
            assert_eq!(get_hero(&pool, variant).await.unwrap(), content);
        }
    }

    #[tokio::test]
    async fn hero_falls_back_on_corrupt_required_values() {
        let pool = pool().await;
        sqlx::query(
            "INSERT INTO hero_content (variant, eyebrow, title, lede, primary_label, primary_href) \
             VALUES ('swiss', '', 'not-json', '', '', '')",
        )
        .execute(&pool)
        .await
        .unwrap();

        let content = get_hero(&pool, SiteVariant::Swiss).await.unwrap();
        let defaults = HeroContent::defaults(SiteVariant::Swiss);

        assert_eq!(content.eyebrow, defaults.eyebrow);
        assert_eq!(content.title, defaults.title);
        assert_eq!(content.lede, defaults.lede);
        assert_eq!(content.primary_label, defaults.primary_label);
        assert_eq!(content.primary_href, defaults.primary_href);
    }
}
