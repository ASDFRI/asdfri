/// 用户实体
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    /// 主键
    pub id: i64,
    /// 密码哈希（Argon2 PHC 字符串）
    pub password: String,
    /// 角色
    pub role: String,
    /// 登录用户名
    pub username: String,
}
