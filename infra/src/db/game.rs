use domain::{Game, GameDraft};
use sqlx::{FromRow, SqlitePool};

use crate::{
    db::{decode_list, encode_list},
    error::AppError,
};

/// `game` 表行映射
#[derive(FromRow)]
struct GameRow {
    id: i64,
    name: String,
    screenshots: String,
    description: Option<String>,
    tags: String,
    link: Option<String>,
    sort_order: i64,
}

impl From<GameRow> for Game {
    fn from(row: GameRow) -> Self {
        Self {
            id: row.id,
            name: row.name,
            screenshots: decode_list(&row.screenshots),
            description: row.description,
            tags: decode_list(&row.tags),
            link: row.link,
            sort_order: row.sort_order,
        }
    }
}

/// 查询全部游戏，按展示排序升序、主键降序
pub async fn list(pool: &SqlitePool) -> Result<Vec<Game>, AppError> {
    let games = sqlx::query_as::<_, GameRow>(
        "SELECT id, name, screenshots, description, tags, link, sort_order FROM game \
         ORDER BY sort_order ASC, id DESC",
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(Game::from)
    .collect();

    Ok(games)
}

/// 按主键查询游戏
pub async fn find(pool: &SqlitePool, id: i64) -> Result<Option<Game>, AppError> {
    let game = sqlx::query_as::<_, GameRow>(
        "SELECT id, name, screenshots, description, tags, link, sort_order FROM game WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .map(Game::from);

    Ok(game)
}

/// 创建游戏并返回新主键
pub async fn create(pool: &SqlitePool, draft: &GameDraft) -> Result<i64, AppError> {
    let result = sqlx::query(
        "INSERT INTO game (name, screenshots, description, tags, link, sort_order) \
         VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(&draft.name)
    .bind(encode_list(&draft.screenshots))
    .bind(&draft.description)
    .bind(encode_list(&draft.tags))
    .bind(&draft.link)
    .bind(draft.sort_order.unwrap_or(0))
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

/// 更新游戏；返回是否命中记录
pub async fn update(pool: &SqlitePool, id: i64, draft: &GameDraft) -> Result<bool, AppError> {
    let result = sqlx::query(
        "UPDATE game SET name = ?, screenshots = ?, description = ?, tags = ?, link = ?, \
         sort_order = COALESCE(?, sort_order) WHERE id = ?",
    )
    .bind(&draft.name)
    .bind(encode_list(&draft.screenshots))
    .bind(&draft.description)
    .bind(encode_list(&draft.tags))
    .bind(&draft.link)
    .bind(draft.sort_order)
    .bind(id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

/// 删除游戏；返回是否命中记录
pub async fn delete(pool: &SqlitePool, id: i64) -> Result<bool, AppError> {
    let result = sqlx::query("DELETE FROM game WHERE id = ?").bind(id).execute(pool).await?;

    Ok(result.rows_affected() > 0)
}

/// 在单个事务中批量更新展示排序
pub async fn reorder(pool: &SqlitePool, orders: &[(i64, i64)]) -> Result<(), AppError> {
    // 1) 事务保证批量写入的原子性
    let mut transaction = pool.begin().await?;

    // 2) 逐条绑定更新，兼容 SQLite 的参数映射
    for (id, sort_order) in orders {
        sqlx::query("UPDATE game SET sort_order = ? WHERE id = ?")
            .bind(sort_order)
            .bind(id)
            .execute(&mut *transaction)
            .await?;
    }

    // 3) 提交全部变更
    transaction.commit().await?;

    Ok(())
}
