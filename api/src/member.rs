use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use server_fn::codec::Json;

#[cfg(feature = "ssr")]
use crate::error::IntoServerFn;

/// 团队成员视图模型
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Member {
    /// 主键
    pub id: i64,
    /// 成员姓名
    pub name: String,
    /// 头像地址
    pub icon: Option<String>,
    /// 详细描述
    pub description: Option<String>,
    /// 个性签名
    pub slogan: Option<String>,
    /// 领域标签
    #[serde(default)]
    pub tags: Vec<String>,
    /// 组织职位
    pub position: Option<String>,
    /// 展示排序
    pub sort_order: i64,
}

/// 创建或更新成员的载荷
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemberPayload {
    /// 成员姓名
    pub name: String,
    /// 头像地址
    pub icon: Option<String>,
    /// 详细描述
    pub description: Option<String>,
    /// 个性签名
    pub slogan: Option<String>,
    /// 领域标签
    #[serde(default)]
    pub tags: Vec<String>,
    /// 组织职位
    pub position: Option<String>,
    /// 展示排序；更新时为空表示保留原值
    pub sort_order: Option<i64>,
}

#[cfg(feature = "ssr")]
impl From<domain::Member> for Member {
    fn from(member: domain::Member) -> Self {
        Self {
            id: member.id,
            name: member.name,
            icon: member.icon,
            description: member.description,
            slogan: member.slogan,
            tags: member.tags,
            position: member.position,
            sort_order: member.sort_order,
        }
    }
}

#[cfg(feature = "ssr")]
impl MemberPayload {
    /// 转换为领域输入，交由领域层清洗与校验
    fn into_draft(self) -> domain::MemberDraft {
        domain::MemberDraft {
            name: self.name,
            icon: self.icon,
            description: self.description,
            slogan: self.slogan,
            tags: self.tags,
            position: self.position,
            sort_order: self.sort_order,
        }
    }
}

/// 获取全部成员，按展示排序返回
#[server]
pub async fn get_members() -> Result<Vec<Member>, ServerFnError> {
    let state = infra::context::state().into_server_fn()?;
    let members = infra::db::member::list(&state.pool).await.into_server_fn()?;

    Ok(members.into_iter().map(Member::from).collect())
}

/// 按主键获取单个成员
#[server(input = Json)]
pub async fn get_member(id: i64) -> Result<Member, ServerFnError> {
    let state = infra::context::state().into_server_fn()?;
    let member = infra::db::member::find(&state.pool, id)
        .await
        .into_server_fn()?
        .ok_or(domain::DomainError::NotFound("成员"))?;

    Ok(Member::from(member))
}

/// 创建成员（需要授权）
#[server(input = Json)]
pub async fn create_member(payload: MemberPayload) -> Result<i64, ServerFnError> {
    // 1) 校验管理员权限
    infra::context::require_admin().await.into_server_fn()?;

    // 2) 领域层清洗与校验
    let draft = payload.into_draft().normalized();
    draft.validate()?;

    // 3) 写入数据库
    let state = infra::context::state().into_server_fn()?;
    infra::db::member::create(&state.pool, &draft).await.into_server_fn()
}

/// 更新成员（需要授权）
#[server(input = Json)]
pub async fn update_member(id: i64, payload: MemberPayload) -> Result<(), ServerFnError> {
    // 1) 校验管理员权限
    infra::context::require_admin().await.into_server_fn()?;

    // 2) 领域层清洗与校验
    let draft = payload.into_draft().normalized();
    draft.validate()?;

    // 3) 更新并校验命中记录
    let state = infra::context::state().into_server_fn()?;
    if !infra::db::member::update(&state.pool, id, &draft).await.into_server_fn()? {
        return Err(domain::DomainError::NotFound("成员").into());
    }

    Ok(())
}

/// 删除成员（需要授权）
#[server(input = Json)]
pub async fn delete_member(id: i64) -> Result<(), ServerFnError> {
    // 1) 校验管理员权限
    infra::context::require_admin().await.into_server_fn()?;

    // 2) 删除并校验命中记录
    let state = infra::context::state().into_server_fn()?;
    if !infra::db::member::delete(&state.pool, id).await.into_server_fn()? {
        return Err(domain::DomainError::NotFound("成员").into());
    }

    Ok(())
}

/// 批量更新成员展示排序（需要授权）
#[server(input = Json)]
pub async fn update_member_orders(orders: Vec<(i64, i64)>) -> Result<(), ServerFnError> {
    // 1) 校验管理员权限
    infra::context::require_admin().await.into_server_fn()?;

    // 2) 事务内批量写入
    let state = infra::context::state().into_server_fn()?;
    infra::db::member::reorder(&state.pool, &orders).await.into_server_fn()
}
