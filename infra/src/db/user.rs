use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::SaltString};
use domain::User;
use sqlx::{FromRow, SqlitePool};
use uuid::Uuid;

use crate::error::AppError;

/// `user` 表行映射
#[derive(FromRow)]
struct UserRow {
    id: i64,
    password: String,
    role: String,
    username: String,
}

impl From<UserRow> for User {
    fn from(row: UserRow) -> Self {
        Self {
            id: row.id,
            password: row.password,
            role: row.role,
            username: row.username,
        }
    }
}

/// 按用户名查询用户
pub async fn find_by_username(pool: &SqlitePool, username: &str) -> Result<Option<User>, AppError> {
    let user = sqlx::query_as::<_, UserRow>(
        "SELECT id, password, role, username FROM user WHERE username = ?",
    )
    .bind(username)
    .fetch_optional(pool)
    .await?
    .map(User::from);

    Ok(user)
}

/// 将历史明文密码升级为 Argon2 哈希
///
/// 迁移中的默认账户以明文写入，首次启动时在此完成一次性升级。
pub async fn upgrade_plaintext_passwords(pool: &SqlitePool) -> Result<(), AppError> {
    let rows =
        sqlx::query_as::<_, (i64, String)>("SELECT id, password FROM user").fetch_all(pool).await?;

    for (id, password) in rows {
        if password.starts_with("$argon2") {
            continue;
        }

        let hash = hash_password(&password)?;
        sqlx::query("UPDATE user SET password = ? WHERE id = ?")
            .bind(hash)
            .bind(id)
            .execute(pool)
            .await?;

        tracing::info!(user_id = id, "已将明文密码升级为 Argon2 哈希");
    }

    Ok(())
}

/// 生成 Argon2 密码哈希
pub fn hash_password(password: &str) -> Result<String, AppError> {
    // 使用 UUID v4 的 16 字节随机值作为盐，避免额外的随机数依赖
    let salt_bytes = Uuid::new_v4().into_bytes();
    let salt = SaltString::encode_b64(&salt_bytes)
        .map_err(|error| AppError::Internal(format!("密码盐生成失败：{error}")))?;

    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|error| AppError::Internal(format!("密码哈希失败：{error}")))
}

/// 校验密码与存储的 Argon2 哈希
pub fn verify_password(password: &str, stored: &str) -> bool {
    let Ok(parsed) = PasswordHash::new(stored) else {
        return false;
    };

    Argon2::default().verify_password(password.as_bytes(), &parsed).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashes_and_verifies_password() {
        let hash = hash_password("secret").unwrap();

        assert!(hash.starts_with("$argon2"));
        assert!(verify_password("secret", &hash));
        assert!(!verify_password("wrong", &hash));
    }

    #[test]
    fn rejects_plaintext_storage() {
        assert!(!verify_password("secret", "secret"));
    }
}
