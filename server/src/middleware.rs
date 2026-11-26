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

/// 依据请求路径与响应类型选择 `Cache-Control` 策略
///
/// 构建产物只有在文件名携带内容哈希时才能长期缓存；否则必须重新验证，
/// 避免浏览器继续使用旧版 wasm 并与新页面组合导致水合失败。
fn cache_control(path: &str, hashed_assets: bool, is_html: bool) -> &'static str {
    // 1) 服务端函数与控制台可能返回按会话授权的内容，禁止任何缓存
    if path.starts_with("/api/") || path == "/console" || path.starts_with("/console/") {
        return "no-store";
    }

    // 2) 上传文件名由 UUID 生成且内容不可变
    if path.starts_with("/uploads/") {
        return "public, max-age=31536000, immutable";
    }

    // 3) 构建产物：哈希文件名可永久缓存，snippets 等未哈希文件仍须验证
    if path.starts_with("/pkg/") {
        return if hashed_assets && !path.starts_with("/pkg/snippets/") {
            "public, max-age=31536000, immutable"
        } else {
            "no-cache"
        };
    }

    // 4) SSR 文档须每次验证；其余静态资源（图片、视频等）允许短时缓存
    if is_html {
        "no-cache"
    } else {
        "public, max-age=3600"
    }
}

/// 按路径为响应写入缓存策略
///
/// 仅缓存成功响应与 304；错误响应（含不存在的 `/pkg` 资源）一律禁止缓存，
/// 避免把 404/500 当作长期资源或页面固定下来。
async fn apply_cache_control(hashed_assets: bool, req: Request, next: Next) -> Response {
    let path = req.uri().path().to_owned();
    let mut response = next.run(req).await;
    let status = response.status();
    let is_html = response
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| value.starts_with("text/html"));

    let policy = if status.is_success() || status == StatusCode::NOT_MODIFIED {
        cache_control(&path, hashed_assets, is_html)
    } else {
        "no-store"
    };

    response.headers_mut().insert(header::CACHE_CONTROL, HeaderValue::from_static(policy));
    response
}

/// 为路由挂载通用中间件
pub fn with_layers<S>(router: Router<S>, hashed_assets: bool) -> Router<S>
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
        .layer(middleware::from_fn(move |req, next| apply_cache_control(hashed_assets, req, next)))
        .layer(middleware::from_fn(reject_pathless_target))
}

#[cfg(test)]
mod tests {
    use axum::{Router, body::Body, http::Request};
    use tower::ServiceExt;

    use super::*;

    /// 以指定请求目标探测完整中间件栈，返回最终状态码
    async fn probe(uri: &str) -> StatusCode {
        let app = with_layers(Router::new().fallback(|| async { StatusCode::OK }), false);

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

    #[test]
    fn hashed_build_assets_are_immutable() {
        assert_eq!(
            cache_control("/pkg/asdfri.abc123.js", true, false),
            "public, max-age=31536000, immutable"
        );
        assert_eq!(cache_control("/pkg/asdfri.js", false, false), "no-cache");
        assert_eq!(cache_control("/pkg/snippets/x.js", true, false), "no-cache");
    }

    #[test]
    fn paths_map_to_distinct_cache_policies() {
        assert_eq!(cache_control("/", true, true), "no-cache");
        assert_eq!(cache_control("/api/login", false, false), "no-store");
        assert_eq!(cache_control("/console", true, true), "no-store");
        assert_eq!(cache_control("/console/theme", true, true), "no-store");
        assert_eq!(cache_control("/console-extra", true, true), "no-cache");
        assert_eq!(
            cache_control("/uploads/x.png", false, false),
            "public, max-age=31536000, immutable"
        );
        assert_eq!(cache_control("/ASDF.mp4", false, false), "public, max-age=3600");
    }

    #[tokio::test]
    async fn writes_cache_control_header() {
        let app = with_layers(Router::new().fallback(|| async { StatusCode::OK }), false);

        let response = app
            .oneshot(Request::builder().uri("/asset.png").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.headers().get(header::CACHE_CONTROL).unwrap(), "public, max-age=3600");
    }

    #[tokio::test]
    async fn never_caches_error_responses() {
        let app = with_layers(Router::new().fallback(|| async { StatusCode::NOT_FOUND }), true);

        let response = app
            .oneshot(Request::builder().uri("/pkg/asdfri.missing.js").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.headers().get(header::CACHE_CONTROL).unwrap(), "no-store");
    }

    #[tokio::test]
    async fn revalidated_assets_keep_immutable_policy() {
        let app = with_layers(Router::new().fallback(|| async { StatusCode::NOT_MODIFIED }), true);

        let response = app
            .oneshot(Request::builder().uri("/pkg/asdfri.abc.js").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(
            response.headers().get(header::CACHE_CONTROL).unwrap(),
            "public, max-age=31536000, immutable"
        );
    }
}
