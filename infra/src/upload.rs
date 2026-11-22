//! 上传文件落盘：按魔数识别图片类型并生成不可预测的文件名。

use std::path::Path;

use uuid::Uuid;

use crate::error::AppError;

/// 上传文件访问路径前缀
pub const UPLOAD_URL_PREFIX: &str = "uploads";
/// 单个文件大小上限（16 MiB）
pub const MAX_UPLOAD_BYTES: usize = 16 * 1024 * 1024;

/// 保存图片并返回可访问的站内地址
///
/// 扩展名由文件内容魔数决定，忽略客户端提供的文件名，避免路径穿越与伪造类型。
pub async fn save_image(upload_dir: &Path, bytes: &[u8]) -> Result<String, AppError> {
    // 1) 零信任校验：拒绝空文件与超大文件
    if bytes.is_empty() {
        return Err(AppError::Domain(domain::DomainError::InvalidInput(
            "上传文件为空".to_string(),
        )));
    }
    if bytes.len() > MAX_UPLOAD_BYTES {
        return Err(AppError::Domain(domain::DomainError::InvalidInput(
            "上传文件超过 16 MiB".to_string(),
        )));
    }

    // 2) 仅接受受支持的图片格式
    let extension = detect_extension(bytes).ok_or_else(|| {
        AppError::Domain(domain::DomainError::InvalidInput(
            "仅支持 PNG、JPEG、GIF、WebP 与 AVIF 图片".to_string(),
        ))
    })?;

    // 3) 生成隔离目录与随机文件名
    let directory = upload_dir;
    tokio::fs::create_dir_all(directory)
        .await
        .map_err(|error| AppError::Internal(format!("创建上传目录失败：{error}")))?;
    let filename = format!("{}.{extension}", Uuid::new_v4().simple());

    // 4) 写入文件并返回站内地址
    tokio::fs::write(directory.join(&filename), bytes)
        .await
        .map_err(|error| AppError::Internal(format!("写入上传文件失败：{error}")))?;

    Ok(format!("/{UPLOAD_URL_PREFIX}/{filename}"))
}

/// 依据魔数识别图片扩展名
fn detect_extension(bytes: &[u8]) -> Option<&'static str> {
    if bytes.starts_with(&[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]) {
        return Some("png");
    }
    if bytes.starts_with(&[0xFF, 0xD8, 0xFF]) {
        return Some("jpg");
    }
    if bytes.starts_with(b"GIF87a") || bytes.starts_with(b"GIF89a") {
        return Some("gif");
    }
    if bytes.len() >= 12 && bytes.starts_with(b"RIFF") && &bytes[8..12] == b"WEBP" {
        return Some("webp");
    }
    if bytes.len() >= 12 && &bytes[4..8] == b"ftyp" && bytes[8..12].starts_with(b"avif") {
        return Some("avif");
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_supported_signatures() {
        assert_eq!(
            detect_extension(&[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]),
            Some("png")
        );
        assert_eq!(detect_extension(&[0xFF, 0xD8, 0xFF, 0x00]), Some("jpg"));
        assert_eq!(detect_extension(b"GIF89a....."), Some("gif"));
        assert_eq!(detect_extension(b"RIFF0000WEBP"), Some("webp"));
        assert_eq!(detect_extension(b"0000ftypavif"), Some("avif"));
    }

    #[test]
    fn rejects_unknown_signatures() {
        assert_eq!(detect_extension(b"<svg></svg>"), None);
        assert_eq!(detect_extension(b""), None);
    }
}
