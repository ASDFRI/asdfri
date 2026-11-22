use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use domain::DomainError;
use serde_json::json;

/// 应用统一错误类型
///
/// 作为 Axum 处理器错误时经 [`IntoResponse`] 输出对应状态码与 JSON；
/// 作为 Server Function 返回值时按 Leptos 约定收敛为 `ServerFnError`。
/// 内部错误只向客户端暴露概要，完整细节写入服务端日志。
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    Domain(#[from] DomainError),
    #[error("配置错误：{0}")]
    Config(String),
    #[error("数据库错误")]
    Database(#[from] sqlx::Error),
    #[error("数据库迁移失败")]
    Migration(#[from] sqlx::migrate::MigrateError),
    #[error("令牌错误")]
    Token(#[from] jsonwebtoken::errors::Error),
    #[error("内部错误")]
    Internal(String),
}

impl AppError {
    /// 错误对应的 HTTP 状态码
    pub fn status(&self) -> StatusCode {
        match self {
            Self::Domain(error) => match error {
                DomainError::MissingCredentials | DomainError::InvalidInput(_) => {
                    StatusCode::BAD_REQUEST
                }
                DomainError::NotFound(_) => StatusCode::NOT_FOUND,
                DomainError::WrongCredentials
                | DomainError::MissingToken
                | DomainError::InvalidToken => StatusCode::UNAUTHORIZED,
                DomainError::Forbidden => StatusCode::FORBIDDEN,
            },
            Self::Config(_)
            | Self::Database(_)
            | Self::Migration(_)
            | Self::Token(_)
            | Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // 1) 服务端错误记录完整细节，客户端只收到概要信息
        let status = self.status();
        if status.is_server_error() {
            tracing::error!(error = ?self, "请求处理失败");
        }

        // 2) 输出统一 JSON 错误体
        (status, Json(json!({ "error": self.to_string() }))).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn domain_errors_remain_descriptive() {
        let error = AppError::Domain(DomainError::Forbidden);

        assert_eq!(error.status(), StatusCode::FORBIDDEN);
        assert_eq!(error.to_string(), "无权执行该操作");
    }

    #[test]
    fn internal_errors_are_sanitized() {
        let database = AppError::Database(sqlx::Error::RowNotFound);
        let internal = AppError::Internal("sensitive detail".to_string());

        assert_eq!(database.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(database.to_string(), "数据库错误");
        assert_eq!(internal.to_string(), "内部错误");
    }
}
