//! 接口层：客户端与服务端共用的 Server Functions。
//!
//! 本 crate 只承载 RPC 契约与传输无关的 DTO，具体实现位于 `infra`。

pub mod auth;
#[cfg(feature = "ssr")]
mod error;
pub mod game;
pub mod member;
pub mod site;
pub mod upload;
pub mod user;
