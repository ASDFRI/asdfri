//! 站点视觉变体与首页首屏内容的读写契约。

pub use domain::{HeroContent, SiteVariant};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use server_fn::codec::Json;

#[cfg(feature = "ssr")]
use crate::error::IntoServerFn;

/// 更新首页首屏内容时提交的载荷
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeroPayload {
    /// 眉题
    pub eyebrow: String,
    /// 地点标记
    pub location: String,
    /// 主标题行；第二行使用强调色
    #[serde(default)]
    pub title: Vec<String>,
    /// 引导文本
    pub lede: String,
    /// 主操作文案
    pub primary_label: String,
    /// 主操作链接
    pub primary_href: String,
    /// 次操作文案；与链接同时留空时隐藏
    pub secondary_label: String,
    /// 次操作链接；与文案同时留空时隐藏
    pub secondary_href: String,
    /// 媒体角标
    pub media_tag: String,
    /// 视频封面地址
    pub media_poster: String,
    /// 视频地址；留空时隐藏媒体
    pub media_video: String,
    /// 底部滚动标签
    #[serde(default)]
    pub strip: Vec<String>,
}

impl From<HeroContent> for HeroPayload {
    fn from(content: HeroContent) -> Self {
        Self {
            eyebrow: content.eyebrow,
            location: content.location,
            title: content.title,
            lede: content.lede,
            primary_label: content.primary_label,
            primary_href: content.primary_href,
            secondary_label: content.secondary_label,
            secondary_href: content.secondary_href,
            media_tag: content.media_tag,
            media_poster: content.media_poster,
            media_video: content.media_video,
            strip: content.strip,
        }
    }
}

#[cfg(feature = "ssr")]
impl HeroPayload {
    /// 转换为领域输入，交由领域层清洗与校验
    fn into_draft(self) -> domain::HeroDraft {
        domain::HeroDraft {
            eyebrow: self.eyebrow,
            location: self.location,
            title: self.title,
            lede: self.lede,
            primary_label: self.primary_label,
            primary_href: self.primary_href,
            secondary_label: self.secondary_label,
            secondary_href: self.secondary_href,
            media_tag: self.media_tag,
            media_poster: self.media_poster,
            media_video: self.media_video,
            strip: self.strip,
        }
    }
}

/// 读取当前站点视觉变体
#[server]
pub async fn get_site_variant() -> Result<SiteVariant, ServerFnError> {
    let state = infra::context::state().into_server_fn()?;

    infra::db::site::get_variant(&state.pool).await.into_server_fn()
}

/// 更新站点视觉变体（需要授权）
#[server(input = Json)]
pub async fn update_site_variant(variant: SiteVariant) -> Result<(), ServerFnError> {
    // 1) 校验管理员权限
    infra::context::require_admin().await.into_server_fn()?;

    // 2) 写入数据库后同步进程内缓存
    let state = infra::context::state().into_server_fn()?;
    infra::db::site::set_variant(&state.pool, variant).await.into_server_fn()?;
    state.theme.set(variant);

    Ok(())
}

/// 读取指定视觉变体的首页首屏内容
#[server(input = Json)]
pub async fn get_hero(variant: SiteVariant) -> Result<HeroContent, ServerFnError> {
    let state = infra::context::state().into_server_fn()?;

    infra::db::site::get_hero(&state.pool, variant).await.into_server_fn()
}

/// 更新指定视觉变体的首页首屏内容（需要授权）
#[server(input = Json)]
pub async fn update_hero(variant: SiteVariant, payload: HeroPayload) -> Result<(), ServerFnError> {
    // 1) 校验管理员权限
    infra::context::require_admin().await.into_server_fn()?;

    // 2) 领域层清洗与校验
    let draft = payload.into_draft().normalized();
    draft.validate()?;

    // 3) 写入数据库
    let state = infra::context::state().into_server_fn()?;
    infra::db::site::set_hero(&state.pool, variant, &draft).await.into_server_fn()?;

    Ok(())
}
