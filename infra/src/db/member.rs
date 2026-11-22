use domain::{Member, MemberDraft};
use sqlx::{FromRow, SqlitePool};

use crate::{
    db::{decode_list, encode_list},
    error::AppError,
};

/// `member` 表行映射
#[derive(FromRow)]
struct MemberRow {
    id: i64,
    name: String,
    icon: Option<String>,
    description: Option<String>,
    slogan: Option<String>,
    tags: String,
    position: Option<String>,
    sort_order: i64,
}

impl From<MemberRow> for Member {
    fn from(row: MemberRow) -> Self {
        Self {
            id: row.id,
            name: row.name,
            icon: row.icon,
            description: row.description,
            slogan: row.slogan,
            tags: decode_list(&row.tags),
            position: row.position,
            sort_order: row.sort_order,
        }
    }
}

/// 查询全部成员，按展示排序升序、主键降序
pub async fn list(pool: &SqlitePool) -> Result<Vec<Member>, AppError> {
    let members = sqlx::query_as::<_, MemberRow>(
        "SELECT id, name, icon, description, slogan, tags, position, sort_order FROM member \
         ORDER BY sort_order ASC, id DESC",
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(Member::from)
    .collect();

    Ok(members)
}

/// 按主键查询成员
pub async fn find(pool: &SqlitePool, id: i64) -> Result<Option<Member>, AppError> {
    let member = sqlx::query_as::<_, MemberRow>(
        "SELECT id, name, icon, description, slogan, tags, position, sort_order FROM member \
         WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .map(Member::from);

    Ok(member)
}

/// 创建成员并返回新主键
pub async fn create(pool: &SqlitePool, draft: &MemberDraft) -> Result<i64, AppError> {
    let result = sqlx::query(
        "INSERT INTO member (name, icon, description, slogan, tags, position, sort_order) \
         VALUES (?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&draft.name)
    .bind(&draft.icon)
    .bind(&draft.description)
    .bind(&draft.slogan)
    .bind(encode_list(&draft.tags))
    .bind(&draft.position)
    .bind(draft.sort_order.unwrap_or(0))
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

/// 更新成员；返回是否命中记录
pub async fn update(pool: &SqlitePool, id: i64, draft: &MemberDraft) -> Result<bool, AppError> {
    let result = sqlx::query(
        "UPDATE member SET name = ?, icon = ?, description = ?, slogan = ?, tags = ?, \
         position = ?, sort_order = COALESCE(?, sort_order) WHERE id = ?",
    )
    .bind(&draft.name)
    .bind(&draft.icon)
    .bind(&draft.description)
    .bind(&draft.slogan)
    .bind(encode_list(&draft.tags))
    .bind(&draft.position)
    .bind(draft.sort_order)
    .bind(id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

/// 删除成员；返回是否命中记录
pub async fn delete(pool: &SqlitePool, id: i64) -> Result<bool, AppError> {
    let result = sqlx::query("DELETE FROM member WHERE id = ?").bind(id).execute(pool).await?;

    Ok(result.rows_affected() > 0)
}

/// 在单个事务中批量更新展示排序
pub async fn reorder(pool: &SqlitePool, orders: &[(i64, i64)]) -> Result<(), AppError> {
    // 1) 事务保证批量写入的原子性
    let mut transaction = pool.begin().await?;

    // 2) 逐条绑定更新，兼容 SQLite 的参数映射
    for (id, sort_order) in orders {
        sqlx::query("UPDATE member SET sort_order = ? WHERE id = ?")
            .bind(sort_order)
            .bind(id)
            .execute(&mut *transaction)
            .await?;
    }

    // 3) 提交全部变更
    transaction.commit().await?;

    Ok(())
}
