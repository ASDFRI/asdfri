use leptos::prelude::*;

/// 内容面板
#[component]
pub fn Panel(
    /// 是否使用深色反白面板
    #[prop(optional)]
    dark: bool,
    /// 附加样式类
    #[prop(into, optional)]
    class: String,
    /// 面板内容
    children: Children,
) -> impl IntoView {
    let tone = if dark {
        "panel--dark"
    } else {
        ""
    };

    view! { <div class=format!("panel {} {}", tone, class)>{children()}</div> }
}
