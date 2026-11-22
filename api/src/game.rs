use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use server_fn::codec::Json;

#[cfg(feature = "ssr")]
use crate::error::IntoServerFn;

/// 游戏作品视图模型
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Game {
    /// 主键
    pub id: i64,
    /// 作品名称
    pub name: String,
    /// 截图地址列表
    #[serde(default)]
    pub screenshots: Vec<String>,
    /// 作品简介
    pub description: Option<String>,
    /// 分类标签
    #[serde(default)]
    pub tags: Vec<String>,
    /// 外部链接
    pub link: Option<String>,
    /// 展示排序
    pub sort_order: i64,
}

/// 创建或更新游戏的载荷
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GamePayload {
    /// 作品名称
    pub name: String,
    /// 截图地址列表
    #[serde(default)]
    pub screenshots: Vec<String>,
    /// 作品简介
    pub description: Option<String>,
    /// 分类标签
    #[serde(default)]
    pub tags: Vec<String>,
    /// 外部链接
    pub link: Option<String>,
    /// 展示排序；更新时为空表示保留原值
    pub sort_order: Option<i64>,
}

#[cfg(feature = "ssr")]
impl From<domain::Game> for Game {
    fn from(game: domain::Game) -> Self {
        Self {
            id: game.id,
            name: game.name,
            screenshots: game.screenshots,
            description: game.description,
            tags: game.tags,
            link: game.link,
            sort_order: game.sort_order,
        }
    }
}

#[cfg(feature = "ssr")]
impl GamePayload {
    /// 转换为领域输入，交由领域层清洗与校验
    fn into_draft(self) -> domain::GameDraft {
        domain::GameDraft {
            name: self.name,
            screenshots: self.screenshots,
            description: self.description,
            tags: self.tags,
            link: self.link,
            sort_order: self.sort_order,
        }
    }
}

/// 获取全部游戏，按展示排序返回
#[server]
pub async fn get_games() -> Result<Vec<Game>, ServerFnError> {
    let state = infra::context::state().into_server_fn()?;
    let games = infra::db::game::list(&state.pool).await.into_server_fn()?;

    Ok(games.into_iter().map(Game::from).collect())
}

/// 按主键获取单个游戏
#[server(input = Json)]
pub async fn get_game(id: i64) -> Result<Game, ServerFnError> {
    let state = infra::context::state().into_server_fn()?;
    let game = infra::db::game::find(&state.pool, id)
        .await
        .into_server_fn()?
        .ok_or(domain::DomainError::NotFound("游戏"))?;

    Ok(Game::from(game))
}

/// 创建游戏（需要授权）
#[server(input = Json)]
pub async fn create_game(payload: GamePayload) -> Result<i64, ServerFnError> {
    // 1) 校验管理员权限
    infra::context::require_admin().await.into_server_fn()?;

    // 2) 领域层清洗与校验
    let draft = payload.into_draft().normalized();
    draft.validate()?;

    // 3) 写入数据库
    let state = infra::context::state().into_server_fn()?;
    infra::db::game::create(&state.pool, &draft).await.into_server_fn()
}

/// 更新游戏（需要授权）
#[server(input = Json)]
pub async fn update_game(id: i64, payload: GamePayload) -> Result<(), ServerFnError> {
    // 1) 校验管理员权限
    infra::context::require_admin().await.into_server_fn()?;

    // 2) 领域层清洗与校验
    let draft = payload.into_draft().normalized();
    draft.validate()?;

    // 3) 更新并校验命中记录
    let state = infra::context::state().into_server_fn()?;
    if !infra::db::game::update(&state.pool, id, &draft).await.into_server_fn()? {
        return Err(domain::DomainError::NotFound("游戏").into());
    }

    Ok(())
}

/// 删除游戏（需要授权）
#[server(input = Json)]
pub async fn delete_game(id: i64) -> Result<(), ServerFnError> {
    // 1) 校验管理员权限
    infra::context::require_admin().await.into_server_fn()?;

    // 2) 删除并校验命中记录
    let state = infra::context::state().into_server_fn()?;
    if !infra::db::game::delete(&state.pool, id).await.into_server_fn()? {
        return Err(domain::DomainError::NotFound("游戏").into());
    }

    Ok(())
}

/// 批量更新游戏展示排序（需要授权）
#[server(input = Json)]
pub async fn update_game_orders(orders: Vec<(i64, i64)>) -> Result<(), ServerFnError> {
    // 1) 校验管理员权限
    infra::context::require_admin().await.into_server_fn()?;

    // 2) 事务内批量写入
    let state = infra::context::state().into_server_fn()?;
    infra::db::game::reorder(&state.pool, &orders).await.into_server_fn()
}
