use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use server_fn::codec::{MultipartData, MultipartFormData};

#[cfg(feature = "ssr")]
use crate::error::IntoServerFn;

/// 上传成功后的响应
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UploadResponse {
    /// 是否成功
    pub success: bool,
    /// 站内访问地址
    pub url: Option<String>,
}

/// 上传图片（需要授权）
///
/// 仅接受由魔数识别的 PNG、JPEG、GIF、WebP 与 AVIF，文件名由服务端生成。
#[server(input = MultipartFormData)]
pub async fn upload_file(data: MultipartData) -> Result<UploadResponse, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        handle_upload(data).await
    }

    #[cfg(not(feature = "ssr"))]
    {
        let _ = data;
        Err(server_error("上传仅在服务端可用"))
    }
}

/// 构造服务端错误，固定使用默认自定义错误类型
#[allow(dead_code)]
fn server_error(message: impl Into<String>) -> ServerFnError {
    ServerFnError::ServerError(message.into())
}

/// 逐字段读取 multipart 并落盘
#[cfg(feature = "ssr")]
async fn handle_upload(data: MultipartData) -> Result<UploadResponse, ServerFnError> {
    // 1) 校验管理员权限
    infra::context::require_admin().await.into_server_fn()?;

    // 2) 提取上传目录
    let state = infra::context::state().into_server_fn()?;
    let upload_dir = state.config.upload.dir.clone();

    // 3) 读取 multipart 流
    let mut data = match data.into_inner() {
        Some(inner) => inner,
        None => return Err(server_error("未收到 multipart 数据流")),
    };

    // 4) 仅处理名为 `file` 的字段
    while let Some(field) = data.next_field().await.map_err(|error| {
        tracing::error!(%error, "读取 multipart 表单字段失败");
        server_error("读取表单字段失败")
    })? {
        if field.name() != Some("file") {
            continue;
        }

        let bytes = field.bytes().await.map_err(|error| {
            tracing::error!(%error, "读取上传文件内容失败");
            server_error("读取文件内容失败")
        })?;
        let url = infra::upload::save_image(std::path::Path::new(&upload_dir), &bytes)
            .await
            .into_server_fn()?;

        return Ok(UploadResponse {
            success: true,
            url: Some(url),
        });
    }

    Err(server_error("未找到名为 `file` 的上传字段"))
}
