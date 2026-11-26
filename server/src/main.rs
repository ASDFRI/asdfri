//! 服务端入口：装配配置、数据库、路由与 HTTP/HTTPS 监听。

mod middleware;
mod transport;

use std::{fs, net::SocketAddr, path::PathBuf};

use axum::Router;
use infra::{AppConfig, AppError, AppState, SiteTheme, auth::Keys, db, telemetry};
use leptos::{config::LeptosOptions, prelude::provide_context};
use leptos_axum::{LeptosRoutes, generate_route_list};
use tower_http::services::ServeDir;

use crate::{middleware::with_layers, transport::serve};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1) 初始化日志，守卫需存活至进程退出
    let _guard = telemetry::init();

    // 2) 加载应用配置与 Leptos 选项
    let config = AppConfig::load()?;
    let mut leptos_options = leptos::config::get_configuration(None)?.leptos_options;
    apply_build_hash_options(&mut leptos_options)?;

    // 3) 确保站点根目录与上传目录存在
    fs::create_dir_all(leptos_options.site_root.as_ref())?;
    fs::create_dir_all(&config.upload.dir)?;

    // 4) /pkg 独立挂载，以便保留条件请求头并返回 304
    let pkg_dir =
        PathBuf::from(leptos_options.site_root.as_ref()).join(leptos_options.site_pkg_dir.as_ref());

    // 5) 连接数据库并执行迁移
    let pool = db::connect(&config.database).await?;

    // 6) 读取站点视觉变体，构建 JWT 密钥与全局状态
    let theme = SiteTheme::new(db::site::get_variant(&pool).await?);
    let keys = Keys::new(&config.auth.jwt_secret)?;
    let state = AppState {
        config,
        keys,
        leptos_options: leptos_options.clone(),
        pool,
        theme,
    };

    // 7) 装配 Leptos 路由与服务端上下文
    let routes = generate_route_list(app::App);
    let router = Router::new()
        .nest_service("/uploads", ServeDir::new(&state.config.upload.dir))
        .nest_service("/pkg", ServeDir::new(&pkg_dir).precompressed_gzip().precompressed_br())
        .leptos_routes_with_context(
            &state,
            routes,
            {
                let state = state.clone();
                move || {
                    provide_context(state.clone());
                    provide_context(state.theme.get());
                }
            },
            {
                let leptos_options = leptos_options.clone();
                move || app::shell(leptos_options.clone())
            },
        )
        .fallback(leptos_axum::file_and_error_handler_with_context::<AppState, _>(
            {
                let theme = state.theme.clone();
                move || provide_context(theme.get())
            },
            app::shell,
        ))
        .with_state(state.clone());

    // 8) 挂载中间件并启动监听
    let addr = listen_addr(&state.config, &state.leptos_options)?;
    serve(with_layers(router, state.leptos_options.hash_files), addr, &state.config.server).await
}

/// 恢复 cargo-leptos 构建期注入的资源哈希配置
///
/// `LEPTOS_HASH_FILES` 仅作为构建期环境变量注入，运行时进程不会继承，
/// 须在启动时显式还原；启用哈希时还须校验清单与站点产物一致。
fn apply_build_hash_options(options: &mut LeptosOptions) -> anyhow::Result<()> {
    // 1) 从编译期环境恢复哈希开关与清单文件名
    if let Some(value) = option_env!("LEPTOS_HASH_FILES") {
        options.hash_files = value == "true";
    }
    if let Some(name) = option_env!("LEPTOS_HASH_FILE_NAME") {
        options.hash_file = name.into();
    }

    // 2) 二进制旁存在清单时同样视为启用哈希，避免直接 cargo build 后复用哈希产物却关闭哈希
    let hash_path = std::env::current_exe()?
        .parent()
        .map(|dir| dir.join(&*options.hash_file))
        .unwrap_or_default();
    options.hash_files |= hash_path.exists();

    // 3) 启用哈希时校验清单与站点产物一致，避免 SSR 生成无效资源地址
    if options.hash_files {
        let hash_list = fs::read_to_string(&hash_path).map_err(|error| {
            anyhow::anyhow!(
                "资源哈希已启用，但无法读取 {}：{error}；请将 cargo-leptos 生成的哈希清单与服务器二进制放在同一目录",
                hash_path.display()
            )
        })?;
        let pkg_dir = PathBuf::from(options.site_root.as_ref()).join(options.site_pkg_dir.as_ref());

        for kind in ["js", "wasm", "css"] {
            // 1) 从清单中取出该类型的哈希
            let hash = hash_list
                .lines()
                .filter_map(|line| line.trim().split_once(':'))
                .find_map(|(name, hash)| (name.trim() == kind).then(|| hash.trim()))
                .ok_or_else(|| {
                    anyhow::anyhow!("哈希清单 {} 缺少 `{kind}` 条目", hash_path.display())
                })?;

            // 2) 校验对应文件确实存在于站点产物目录
            let asset = pkg_dir.join(format!("{}.{hash}.{kind}", options.output_name));
            anyhow::ensure!(
                asset.exists(),
                "资源哈希清单与站点产物不一致：缺少 {}；请重新执行完整构建并分发 target/site",
                asset.display()
            );
        }
    }

    Ok(())
}

/// 解析监听地址
///
/// 开发构建沿用 cargo-leptos 注入的 [`LeptosOptions::site_addr`]；
/// release 构建使用应用配置 `[server]`。两条配置链相互独立。
fn listen_addr(config: &AppConfig, leptos_options: &LeptosOptions) -> Result<SocketAddr, AppError> {
    #[cfg(debug_assertions)]
    {
        let _ = config;
        Ok(leptos_options.site_addr)
    }

    #[cfg(not(debug_assertions))]
    {
        let _ = leptos_options;
        config.server.listen_addr()
    }
}
