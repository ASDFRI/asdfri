use crate::{
    error::DomainError,
    game::{clean_list, is_safe_asset, trim_optional},
};

/// 团队成员实体
#[derive(Debug, Clone, PartialEq, Eq)]
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
    pub tags: Vec<String>,
    /// 组织职位
    pub position: Option<String>,
    /// 展示排序，数值越小越靠前
    pub sort_order: i64,
}

/// 创建或更新成员时提交的输入
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemberDraft {
    /// 成员姓名
    pub name: String,
    /// 头像地址
    pub icon: Option<String>,
    /// 详细描述
    pub description: Option<String>,
    /// 个性签名
    pub slogan: Option<String>,
    /// 领域标签
    pub tags: Vec<String>,
    /// 组织职位
    pub position: Option<String>,
    /// 展示排序；更新时为空表示保留原值
    pub sort_order: Option<i64>,
}

impl MemberDraft {
    /// 清洗输入：裁剪空白、丢弃空项并去重
    pub fn normalized(mut self) -> Self {
        self.name = self.name.trim().to_string();
        self.icon = trim_optional(self.icon);
        self.description = trim_optional(self.description);
        self.slogan = trim_optional(self.slogan);
        self.position = trim_optional(self.position);
        self.tags = clean_list(self.tags, 24);

        self
    }

    /// 校验输入是否满足领域约束
    pub fn validate(&self) -> Result<(), DomainError> {
        if self.name.is_empty() {
            return Err(DomainError::InvalidInput("成员姓名不能为空".to_string()));
        }
        if self.name.chars().count() > 80 {
            return Err(DomainError::InvalidInput("成员姓名不能超过 80 个字符".to_string()));
        }
        if self.position.as_ref().is_some_and(|text| text.chars().count() > 80) {
            return Err(DomainError::InvalidInput("组织职位不能超过 80 个字符".to_string()));
        }
        if self.slogan.as_ref().is_some_and(|text| text.chars().count() > 200) {
            return Err(DomainError::InvalidInput("个性签名不能超过 200 个字符".to_string()));
        }
        if self.description.as_ref().is_some_and(|text| text.chars().count() > 4_000) {
            return Err(DomainError::InvalidInput("成员描述不能超过 4000 个字符".to_string()));
        }
        if self.icon.as_ref().is_some_and(|url| url.chars().count() > 2_048) {
            return Err(DomainError::InvalidInput("头像地址过长".to_string()));
        }
        if self.icon.as_deref().is_some_and(|url| !is_safe_asset(url)) {
            return Err(DomainError::InvalidInput(
                "头像地址仅支持 http(s) 或站内绝对路径".to_string(),
            ));
        }
        if self.tags.iter().any(|tag| tag.chars().count() > 64) {
            return Err(DomainError::InvalidInput("单个标签不能超过 64 个字符".to_string()));
        }

        Ok(())
    }
}
