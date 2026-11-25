use serde::{Deserialize, Serialize};

use crate::{
    error::DomainError,
    game::{clean_list, is_safe_asset, is_safe_link},
};

/// 公开站点视觉变体
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum SiteVariant {
    /// 瑞士国际主义：严格网格、克制线条与明确层级
    #[serde(rename = "swiss")]
    #[default]
    Swiss,
    /// 纯构成主义：斜切几何、硬阴影与三色限制
    #[serde(rename = "constructivist")]
    Constructivist,
    /// 史诗叙事：电影画幅、鎏金细节与深空光影
    #[serde(rename = "epic")]
    Epic,
}

impl SiteVariant {
    /// 全部可选变体
    pub const ALL: [Self; 3] = [Self::Swiss, Self::Constructivist, Self::Epic];

    /// 数据库与页面标记使用的稳定标识
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Swiss => "swiss",
            Self::Constructivist => "constructivist",
            Self::Epic => "epic",
        }
    }

    /// 解析稳定标识
    pub fn parse(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "swiss" => Some(Self::Swiss),
            "constructivist" => Some(Self::Constructivist),
            "epic" => Some(Self::Epic),
            _ => None,
        }
    }
}

/// 首页首屏内容
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeroContent {
    /// 眉题
    pub eyebrow: String,
    /// 地点标记
    pub location: String,
    /// 主标题行；第二行使用强调色
    #[serde(default)]
    pub title: Vec<String>,
    /// 引导文本
    pub lede: String,
    /// 主操作文案
    pub primary_label: String,
    /// 主操作链接
    pub primary_href: String,
    /// 次操作文案；与链接同时留空时隐藏
    pub secondary_label: String,
    /// 次操作链接；与文案同时留空时隐藏
    pub secondary_href: String,
    /// 媒体角标
    pub media_tag: String,
    /// 视频封面地址
    pub media_poster: String,
    /// 视频地址；留空时隐藏媒体
    pub media_video: String,
    /// 底部滚动标签
    #[serde(default)]
    pub strip: Vec<String>,
}

impl HeroContent {
    /// 指定视觉变体的默认首屏内容
    pub fn defaults(variant: SiteVariant) -> Self {
        Self {
            eyebrow: "ASDF Studio / 2026".to_string(),
            location: "31°N 114°E · Wuhan".to_string(),
            title: vec!["我们创造".to_string(), "虚拟".to_string(), "世界".to_string()],
            lede: match variant {
                SiteVariant::Swiss => {
                    "一个由梦想家、工程师与艺术家组成的工作室。以结构主义的几何秩序与瑞士国际主义的网格理性，重新组织互动娱乐的表达。"
                }
                SiteVariant::Constructivist => {
                    "一个由梦想家、工程师与艺术家组成的工作室。以构成主义的几何秩序、斜切色块与生产力，重新组织互动娱乐的表达。"
                }
                SiteVariant::Epic => {
                    "一个由梦想家、工程师与艺术家组成的工作室。以电影化的光影、鎏金细节与长篇叙事的耐心，重新组织互动娱乐的表达。"
                }
            }
            .to_string(),
            primary_label: "查看作品".to_string(),
            primary_href: "#games".to_string(),
            secondary_label: "发起合作".to_string(),
            secondary_href: "#contact".to_string(),
            media_tag: "// REEL 2026".to_string(),
            media_poster: "/ASDF.png".to_string(),
            media_video: "/ASDF.mp4".to_string(),
            strip: ["UE5", "RUST", "LEPTOS", "AXUM", "BEVY"]
                .into_iter()
                .map(str::to_string)
                .collect(),
        }
    }
}

/// 更新首页首屏内容时提交的输入
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeroDraft {
    /// 眉题
    pub eyebrow: String,
    /// 地点标记
    pub location: String,
    /// 主标题行；第二行使用强调色
    pub title: Vec<String>,
    /// 引导文本
    pub lede: String,
    /// 主操作文案
    pub primary_label: String,
    /// 主操作链接
    pub primary_href: String,
    /// 次操作文案
    pub secondary_label: String,
    /// 次操作链接
    pub secondary_href: String,
    /// 媒体角标
    pub media_tag: String,
    /// 视频封面地址
    pub media_poster: String,
    /// 视频地址
    pub media_video: String,
    /// 底部滚动标签
    pub strip: Vec<String>,
}

impl From<HeroContent> for HeroDraft {
    fn from(content: HeroContent) -> Self {
        Self {
            eyebrow: content.eyebrow,
            location: content.location,
            title: content.title,
            lede: content.lede,
            primary_label: content.primary_label,
            primary_href: content.primary_href,
            secondary_label: content.secondary_label,
            secondary_href: content.secondary_href,
            media_tag: content.media_tag,
            media_poster: content.media_poster,
            media_video: content.media_video,
            strip: content.strip,
        }
    }
}

impl HeroDraft {
    /// 清洗输入：裁剪空白、丢弃空行并限制列表长度
    pub fn normalized(mut self) -> Self {
        self.eyebrow = self.eyebrow.trim().to_string();
        self.location = self.location.trim().to_string();
        self.title = self
            .title
            .into_iter()
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty())
            .collect();
        self.lede = self.lede.trim().to_string();
        self.primary_label = self.primary_label.trim().to_string();
        self.primary_href = self.primary_href.trim().to_string();
        self.secondary_label = self.secondary_label.trim().to_string();
        self.secondary_href = self.secondary_href.trim().to_string();
        self.media_tag = self.media_tag.trim().to_string();
        self.media_poster = self.media_poster.trim().to_string();
        self.media_video = self.media_video.trim().to_string();
        self.strip = clean_list(self.strip, 8);

        self
    }

    /// 校验输入是否满足领域约束
    pub fn validate(&self) -> Result<(), DomainError> {
        if self.eyebrow.is_empty() {
            return Err(DomainError::InvalidInput("眉题不能为空".to_string()));
        }
        if self.eyebrow.chars().count() > 120 {
            return Err(DomainError::InvalidInput("眉题不能超过 120 个字符".to_string()));
        }
        if self.location.chars().count() > 120 {
            return Err(DomainError::InvalidInput("地点标记不能超过 120 个字符".to_string()));
        }
        if self.title.is_empty() {
            return Err(DomainError::InvalidInput("主标题至少需要一行".to_string()));
        }
        if self.title.len() > 3 {
            return Err(DomainError::InvalidInput("主标题最多三行".to_string()));
        }
        if self.title.iter().any(|line| line.chars().count() > 120) {
            return Err(DomainError::InvalidInput("单行主标题不能超过 120 个字符".to_string()));
        }
        if self.lede.is_empty() {
            return Err(DomainError::InvalidInput("引导文本不能为空".to_string()));
        }
        if self.lede.chars().count() > 2_000 {
            return Err(DomainError::InvalidInput("引导文本不能超过 2000 个字符".to_string()));
        }
        if self.primary_label.is_empty() {
            return Err(DomainError::InvalidInput("主操作文案不能为空".to_string()));
        }
        if self.primary_label.chars().count() > 64 {
            return Err(DomainError::InvalidInput("主操作文案不能超过 64 个字符".to_string()));
        }
        if self.primary_href.is_empty() {
            return Err(DomainError::InvalidInput("主操作链接不能为空".to_string()));
        }
        if self.primary_href.chars().count() > 2_048 {
            return Err(DomainError::InvalidInput("主操作链接过长".to_string()));
        }
        if !is_safe_link(&self.primary_href) {
            return Err(DomainError::InvalidInput(
                "主操作链接仅支持 http(s)、mailto、站内路径或锚点".to_string(),
            ));
        }

        let has_secondary_label = !self.secondary_label.is_empty();
        let has_secondary_href = !self.secondary_href.is_empty();
        if has_secondary_label != has_secondary_href {
            return Err(DomainError::InvalidInput(
                "次操作的文案与链接必须同时填写或同时留空".to_string(),
            ));
        }
        if self.secondary_label.chars().count() > 64 {
            return Err(DomainError::InvalidInput("次操作文案不能超过 64 个字符".to_string()));
        }
        if has_secondary_href {
            if self.secondary_href.chars().count() > 2_048 {
                return Err(DomainError::InvalidInput("次操作链接过长".to_string()));
            }
            if !is_safe_link(&self.secondary_href) {
                return Err(DomainError::InvalidInput(
                    "次操作链接仅支持 http(s)、mailto、站内路径或锚点".to_string(),
                ));
            }
        }

        if self.media_tag.chars().count() > 120 {
            return Err(DomainError::InvalidInput("媒体角标不能超过 120 个字符".to_string()));
        }
        if !self.media_poster.is_empty() {
            if self.media_poster.chars().count() > 2_048 {
                return Err(DomainError::InvalidInput("视频封面地址过长".to_string()));
            }
            if !is_safe_asset(&self.media_poster) {
                return Err(DomainError::InvalidInput(
                    "视频封面仅支持 http(s) 或站内绝对路径".to_string(),
                ));
            }
        }
        if !self.media_video.is_empty() {
            if self.media_video.chars().count() > 2_048 {
                return Err(DomainError::InvalidInput("视频地址过长".to_string()));
            }
            if !is_safe_asset(&self.media_video) {
                return Err(DomainError::InvalidInput(
                    "视频地址仅支持 http(s) 或站内绝对路径".to_string(),
                ));
            }
        }

        if self.strip.len() > 8 {
            return Err(DomainError::InvalidInput("底部滚动标签不能超过 8 个".to_string()));
        }
        if self.strip.iter().any(|label| label.chars().count() > 32) {
            return Err(DomainError::InvalidInput("单个底部标签不能超过 32 个字符".to_string()));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn draft() -> HeroDraft {
        HeroContent::defaults(SiteVariant::Swiss).into()
    }

    #[test]
    fn identifiers_round_trip() {
        for variant in SiteVariant::ALL {
            assert_eq!(SiteVariant::parse(variant.as_str()), Some(variant));
        }
    }

    #[test]
    fn rejects_unknown_identifiers() {
        assert_eq!(SiteVariant::parse("brutalist"), None);
        assert_eq!(SiteVariant::parse(""), None);
    }

    #[test]
    fn defaults_differ_only_in_lede() {
        for (index, left) in SiteVariant::ALL.into_iter().enumerate() {
            for right in SiteVariant::ALL.into_iter().skip(index + 1) {
                let left = HeroContent::defaults(left);
                let right = HeroContent::defaults(right);

                assert_ne!(left.lede, right.lede);
                assert_eq!(left.title, right.title);
                assert_eq!(left.primary_href, right.primary_href);
                assert_eq!(left.media_video, right.media_video);
            }
        }
    }

    #[test]
    fn defaults_pass_validation() {
        for variant in SiteVariant::ALL {
            let draft: HeroDraft = HeroContent::defaults(variant).into();
            assert!(draft.normalized().validate().is_ok());
        }
    }

    #[test]
    fn normalizes_whitespace_empty_lines_and_duplicate_tags() {
        let mut value = draft();
        value.eyebrow = "  ASDF  ".to_string();
        value.title = vec![" 我们创造 ".to_string(), "   ".to_string(), " 虚拟 ".to_string()];
        value.strip = vec![" UE5 ".to_string(), "UE5".to_string(), " RUST ".to_string()];

        let normalized = value.normalized();
        assert_eq!(normalized.eyebrow, "ASDF");
        assert_eq!(normalized.title, ["我们创造", "虚拟"]);
        assert_eq!(normalized.strip, ["UE5", "RUST"]);
    }

    #[test]
    fn rejects_empty_required_fields() {
        let mut value = draft();
        value.eyebrow.clear();
        assert!(value.validate().is_err());

        let mut value = draft();
        value.title.clear();
        assert!(value.validate().is_err());

        let mut value = draft();
        value.lede.clear();
        assert!(value.validate().is_err());

        let mut value = draft();
        value.primary_label.clear();
        assert!(value.validate().is_err());

        let mut value = draft();
        value.primary_href.clear();
        assert!(value.validate().is_err());
    }

    #[test]
    fn rejects_unsafe_primary_links() {
        for link in ["javascript:alert(1)", "data:text/html,boom", "ftp://example.com"] {
            let mut value = draft();
            value.primary_href = link.to_string();
            assert!(value.validate().is_err(), "应当拒绝 {link}");
        }

        for link in ["#games", "/#contact", "https://example.com", "mailto:team@example.com"] {
            let mut value = draft();
            value.primary_href = link.to_string();
            assert!(value.validate().is_ok(), "应当接受 {link}");
        }
    }

    #[test]
    fn rejects_unsafe_media_assets() {
        let mut value = draft();
        value.media_video = "javascript:alert(1)".to_string();
        assert!(value.validate().is_err());

        let mut value = draft();
        value.media_poster = "data:image/png;base64,AAAA".to_string();
        assert!(value.validate().is_err());
    }

    #[test]
    fn requires_secondary_action_pair() {
        let mut value = draft();
        value.secondary_href.clear();
        assert!(value.validate().is_err());

        let mut value = draft();
        value.secondary_label.clear();
        assert!(value.validate().is_err());

        let mut value = draft();
        value.secondary_label.clear();
        value.secondary_href.clear();
        assert!(value.validate().is_ok());
    }

    #[test]
    fn rejects_title_over_three_lines() {
        let mut value = draft();
        value.title = vec!["一".to_string(), "二".to_string(), "三".to_string(), "四".to_string()];

        assert!(value.validate().is_err());
    }
}
