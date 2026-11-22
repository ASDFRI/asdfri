use leptos::prelude::*;

/// 标签视觉层级
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TagVariant {
    /// 线框
    #[default]
    Outline,
    /// 实心
    Solid,
    /// 强调色
    Accent,
}

impl TagVariant {
    /// 对应的样式类
    fn class(self) -> &'static str {
        match self {
            Self::Outline => "tag--outline",
            Self::Solid => "tag--solid",
            Self::Accent => "tag--accent",
        }
    }
}

/// 分类标签
#[component]
pub fn Tag(
    /// 标签文本
    #[prop(into)]
    label: String,
    /// 视觉层级
    #[prop(default = TagVariant::Outline)]
    variant: TagVariant,
    /// 附加样式类
    #[prop(into, optional)]
    class: String,
) -> impl IntoView {
    view! { <span class=format!("tag {} {}", variant.class(), class)>{label}</span> }
}
