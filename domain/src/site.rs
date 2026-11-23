use serde::{Deserialize, Serialize};

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
}

impl SiteVariant {
    /// 全部可选变体
    pub const ALL: [Self; 2] = [Self::Swiss, Self::Constructivist];

    /// 数据库与页面标记使用的稳定标识
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Swiss => "swiss",
            Self::Constructivist => "constructivist",
        }
    }

    /// 解析稳定标识
    pub fn parse(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "swiss" => Some(Self::Swiss),
            "constructivist" => Some(Self::Constructivist),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
