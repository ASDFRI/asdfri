//! Server Function 错误收敛：内部错误记录完整细节，客户端只收到概要。

use leptos::prelude::*;

/// 将应用错误转为 `ServerFnError`，并对服务端错误记录日志
pub(crate) trait IntoServerFn<T> {
    /// 执行收敛
    fn into_server_fn(self) -> Result<T, ServerFnError>;
}

impl<T> IntoServerFn<T> for Result<T, infra::AppError> {
    fn into_server_fn(self) -> Result<T, ServerFnError> {
        self.map_err(|error| {
            if error.status().is_server_error() {
                tracing::error!(error = ?error, "Server Function 执行失败");
            }

            ServerFnError::ServerError(error.to_string())
        })
    }
}
