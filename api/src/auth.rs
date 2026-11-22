use leptos::prelude::*;

#[cfg(feature = "ssr")]
use crate::error::IntoServerFn;

/// 登录：校验凭据并写入会话 Cookie
#[server]
pub async fn login(username: String, password: String) -> Result<(), ServerFnError> {
    infra::auth::login(&username, &password).await.into_server_fn()?;

    Ok(())
}

/// 注销：清除会话 Cookie
#[server]
pub async fn logout() -> Result<(), ServerFnError> {
    infra::auth::logout().into_server_fn()?;

    Ok(())
}
