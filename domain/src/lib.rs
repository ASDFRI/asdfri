//! 领域层：纯业务模型与校验规则，不依赖任何基础设施。

pub mod error;
pub mod game;
pub mod member;
pub mod site;
pub mod user;

pub use error::DomainError;
pub use game::{Game, GameDraft};
pub use member::{Member, MemberDraft};
pub use site::SiteVariant;
pub use user::User;
