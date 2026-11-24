//! HTTP 中间件装配。

use axum::{
    Router,
    extract::{DefaultBodyLimit, Request},
    http::{HeaderValue, StatusCode, header},
    middleware::{self, Next},
    response::{IntoResponse, Response},
};
use tower_http::{limit::RequestBodyLimitLayer, set_header::SetResponseHeaderLayer};

/// 请求体大小上限（250 MiB），用于支撑文件上传等场景
const MAX_BODY_BYTES: usize = 250 * 1024 * 1024;

/// 拦截缺少路径分量的请求目标
///
/// authority-form 目标（如开放代理探测发送的 `CONNECT host:port`）不含路径分量，
/// leptos_axum 的 SSR 处理链会在 `path_and_query().unwrap()` 处 panic；
/// 此类请求须在进入任何处理器之前拒绝。
async fn reject_pathless_target(req: Request, next: Next) -> Response {
    // 1) RFC 9110 规定 origin 服务器应拒绝无法解析的请求目标
    if req.uri().path_and_query().is_none() {
        return StatusCode::BAD_REQUEST.into_response();
    }

    // 2) 其余目标（origin-form、absolute-form、asterisk-form）均携带路径分量，放行
    next.run(req).await
}

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
        .layer(middleware::from_fn(reject_pathless_target))
}

#[cfg(test)]
mod tests {
    use axum::{Router, body::Body, http::Request};
    use tower::ServiceExt;

    use super::*;

    /// 以指定请求目标探测完整中间件栈，返回最终状态码
    async fn probe(uri: &str) -> StatusCode {
        let app = with_layers(Router::new().fallback(|| async { StatusCode::OK }));

        app.oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
            .await
            .unwrap()
            .status()
    }

    #[tokio::test]
    async fn rejects_authority_form_target() {
        assert_eq!(probe("example.com:443").await, StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn passes_origin_form_target() {
        assert_eq!(probe("/path?query=1").await, StatusCode::OK);
    }

    #[tokio::test]
    async fn passes_asterisk_and_absolute_form_target() {
        // 1) asterisk-form 与 absolute-form 均携带路径分量，无需拦截
        assert_eq!(probe("*").await, StatusCode::OK);
        assert_eq!(probe("http://example.com").await, StatusCode::OK);
    }
}
