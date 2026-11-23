//! 站点视觉变体的读写契约。

pub use domain::SiteVariant;
use leptos::prelude::*;
use server_fn::codec::Json;

#[cfg(feature = "ssr")]
use crate::error::IntoServerFn;

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
