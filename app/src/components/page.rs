use leptos::prelude::*;

/// 章节标题：编号、眉题、主标题与引导文本
#[component]
pub fn SectionHead(
    /// 章节编号
    #[prop(into)]
    index: String,
    /// 英文眉题
    #[prop(into)]
    eyebrow: String,
    /// 主标题
    #[prop(into)]
    title: String,
    /// 引导文本
    #[prop(into, default = String::new())]
    lede: String,
) -> impl IntoView {
    let lede_view =
        (!lede.trim().is_empty()).then(|| view! { <p class="section-head__lede">{lede}</p> });

    view! {
        <header class="section-head">
            <span class="section-head__index mono">{index}</span>
            <div class="section-head__text">
                <span class="eyebrow">{eyebrow}</span>
                <h2 class="section-head__title">{title}</h2>
                {lede_view}
            </div>
        </header>
    }
}

/// 带箭头的文本链接
#[component]
pub fn ArrowLink(
    /// 目标地址
    #[prop(into)]
    href: String,
    /// 附加样式类
    #[prop(into, optional)]
    class: String,
    /// 链接文本
    children: Children,
) -> impl IntoView {
    let external = href.starts_with("http://") || href.starts_with("https://");

    view! {
        <a
            href=href
            class=format!("arrow-link {}", class)
            target=external.then_some("_blank")
            rel=external.then_some("noopener noreferrer")
        >
            <span>{children()}</span>
            <span class="arrow-link__mark" aria-hidden="true">"→"</span>
        </a>
    }
}
