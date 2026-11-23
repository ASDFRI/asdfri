use crate::error::DomainError;

/// 游戏作品实体
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Game {
    /// 主键
    pub id: i64,
    /// 作品名称
    pub name: String,
    /// 截图地址列表
    pub screenshots: Vec<String>,
    /// 作品简介
    pub description: Option<String>,
    /// 分类标签
    pub tags: Vec<String>,
    /// 外部链接
    pub link: Option<String>,
    /// 展示排序，数值越小越靠前
    pub sort_order: i64,
}

/// 创建或更新游戏时提交的输入
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameDraft {
    /// 作品名称
    pub name: String,
    /// 截图地址列表
    pub screenshots: Vec<String>,
    /// 作品简介
    pub description: Option<String>,
    /// 分类标签
    pub tags: Vec<String>,
    /// 外部链接
    pub link: Option<String>,
    /// 展示排序；更新时为空表示保留原值
    pub sort_order: Option<i64>,
}

impl GameDraft {
    /// 清洗输入：裁剪空白、丢弃空项并去重
    pub fn normalized(mut self) -> Self {
        self.name = self.name.trim().to_string();
        self.description = trim_optional(self.description);
        self.link = trim_optional(self.link);
        self.screenshots = clean_list(self.screenshots, 32);
        self.tags = clean_list(self.tags, 24);

        self
    }

    /// 校验输入是否满足领域约束
    pub fn validate(&self) -> Result<(), DomainError> {
        if self.name.is_empty() {
            return Err(DomainError::InvalidInput("游戏名称不能为空".to_string()));
        }
        if self.name.chars().count() > 120 {
            return Err(DomainError::InvalidInput("游戏名称不能超过 120 个字符".to_string()));
        }
        if self.description.as_ref().is_some_and(|text| text.chars().count() > 4_000) {
            return Err(DomainError::InvalidInput("游戏简介不能超过 4000 个字符".to_string()));
        }
        if self.link.as_ref().is_some_and(|link| link.chars().count() > 2_048) {
            return Err(DomainError::InvalidInput("外部链接过长".to_string()));
        }
        if self.link.as_deref().is_some_and(|link| !is_safe_link(link)) {
            return Err(DomainError::InvalidInput(
                "外部链接仅支持 http(s)、mailto、站内路径或锚点".to_string(),
            ));
        }
        if self.screenshots.iter().any(|url| url.chars().count() > 2_048) {
            return Err(DomainError::InvalidInput("截图地址过长".to_string()));
        }
        if self.screenshots.iter().any(|url| !is_safe_asset(url)) {
            return Err(DomainError::InvalidInput(
                "截图地址仅支持 http(s) 或站内绝对路径".to_string(),
            ));
        }
        if self.tags.iter().any(|tag| tag.chars().count() > 64) {
            return Err(DomainError::InvalidInput("单个标签不能超过 64 个字符".to_string()));
        }

        Ok(())
    }
}

/// 裁剪可选字符串，空值归一为 `None`
pub(crate) fn trim_optional(value: Option<String>) -> Option<String> {
    value.map(|text| text.trim().to_string()).filter(|text| !text.is_empty())
}

/// 允许站外链接、邮件、站内路径与锚点
pub(crate) fn is_safe_link(value: &str) -> bool {
    let value = value.trim().to_ascii_lowercase();

    value.starts_with("http://")
        || value.starts_with("https://")
        || value.starts_with("mailto:")
        || value.starts_with('/')
        || value.starts_with('#')
}

/// 资源地址仅允许站外 http(s) 与站内绝对路径
pub(crate) fn is_safe_asset(value: &str) -> bool {
    let value = value.trim().to_ascii_lowercase();

    value.starts_with("http://") || value.starts_with("https://") || value.starts_with('/')
}

/// 清洗列表：裁剪、丢弃空项、按首次出现顺序去重并限制长度
pub(crate) fn clean_list(values: Vec<String>, limit: usize) -> Vec<String> {
    let mut output = Vec::new();

    for value in values {
        let value = value.trim().to_string();
        if value.is_empty() || output.contains(&value) {
            continue;
        }
        output.push(value);
        if output.len() == limit {
            break;
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    fn draft() -> GameDraft {
        GameDraft {
            name: "Game".to_string(),
            screenshots: vec!["/Games/test.jpg".to_string()],
            description: None,
            tags: vec![],
            link: Some("https://example.com".to_string()),
            sort_order: None,
        }
    }

    #[test]
    fn accepts_safe_urls() {
        assert!(draft().validate().is_ok());

        let mut value = draft();
        value.link = Some("mailto:team@example.com".to_string());
        assert!(value.validate().is_ok());

        let mut value = draft();
        value.link = Some("/#contact".to_string());
        assert!(value.validate().is_ok());
    }

    #[test]
    fn rejects_unsafe_urls() {
        let mut value = draft();
        value.link = Some("javascript:alert(1)".to_string());
        assert!(value.validate().is_err());

        let mut value = draft();
        value.screenshots = vec!["data:image/png;base64,AAAA".to_string()];
        assert!(value.validate().is_err());
    }
}
