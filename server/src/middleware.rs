//! HTTP 中间件装配。

use axum::{
    Router,
    extract::DefaultBodyLimit,
    http::{HeaderValue, header},
};
use tower_http::{limit::RequestBodyLimitLayer, set_header::SetResponseHeaderLayer};

/// 请求体大小上限（250 MiB），用于支撑文件上传等场景
const MAX_BODY_BYTES: usize = 250 * 1024 * 1024;

/// 为路由挂载通用中间件
pub fn with_layers<S>(router: Router<S>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    router
        .layer(SetResponseHeaderLayer::overriding(
            header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::X_FRAME_OPTIONS,
            HeaderValue::from_static("DENY"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::REFERRER_POLICY,
            HeaderValue::from_static("strict-origin-when-cross-origin"),
        ))
        .layer(DefaultBodyLimit::disable())
        .layer(RequestBodyLimitLayer::new(MAX_BODY_BYTES))
}
