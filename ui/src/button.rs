use leptos::ev::MouseEvent;
use leptos::prelude::*;

/// 按钮视觉层级
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonVariant {
    /// 实心主操作
    #[default]
    Solid,
    /// 强调色主操作
    Accent,
    /// 线框次操作
    Outline,
    /// 无边框文本操作
    Ghost,
    /// 危险操作
    Danger,
}

impl ButtonVariant {
    /// 对应的样式类
    fn class(self) -> &'static str {
        match self {
            Self::Solid => "btn--solid",
            Self::Accent => "btn--accent",
            Self::Outline => "btn--outline",
            Self::Ghost => "btn--ghost",
            Self::Danger => "btn--danger",
        }
    }
}

/// 标准按钮
#[component]
pub fn Button(
    /// 视觉层级
    #[prop(default = ButtonVariant::Solid)]
    variant: ButtonVariant,
    /// 原生按钮类型
    #[prop(into, default = "button".to_string())]
    button_type: String,
    /// 禁用状态
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// 附加样式类
    #[prop(into, optional)]
    class: String,
    /// 点击回调
    #[prop(into, optional)]
    on_click: Option<Callback<MouseEvent>>,
    /// 按钮内容
    children: Children,
) -> impl IntoView {
    view! {
        <button
            type=button_type
            disabled=disabled
            class=format!("btn {} {}", variant.class(), class)
            on:click=move |event| {
                if let Some(callback) = on_click {
                    callback.run(event);
                }
            }
        >
            {children()}
        </button>
    }
}

/// 图标按钮
#[component]
pub fn IconButton(
    /// 视觉层级
    #[prop(default = ButtonVariant::Ghost)]
    variant: ButtonVariant,
    /// 原生按钮类型
    #[prop(into, default = "button".to_string())]
    button_type: String,
    /// 无障碍标签
    #[prop(into, default = String::new())]
    label: String,
    /// 禁用状态
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// 附加样式类
    #[prop(into, optional)]
    class: String,
    /// 点击回调
    #[prop(into, optional)]
    on_click: Option<Callback<MouseEvent>>,
    /// 图标内容
    children: Children,
) -> impl IntoView {
    view! {
        <button
            type=button_type
            aria-label=label
            disabled=disabled
            class=format!("icon-btn {} {}", variant.class(), class)
            on:click=move |event| {
                if let Some(callback) = on_click {
                    callback.run(event);
                }
            }
        >
            {children()}
        </button>
    }
}
