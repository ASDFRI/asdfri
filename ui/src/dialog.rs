use leptos::prelude::*;

/// 模态对话框
#[component]
pub fn Dialog(
    /// 开关状态
    open: RwSignal<bool>,
    /// 标题
    #[prop(into)]
    title: String,
    /// 说明文本
    #[prop(into, default = String::new())]
    description: String,
    /// 附加样式类
    #[prop(into, optional)]
    class: String,
    /// 对话框内容
    children: Children,
) -> impl IntoView {
    let has_description = !description.trim().is_empty();
    let title_label = title.clone();

    view! {
        <div
            class=move || format!("dialog-backdrop {}", if open.get() { "is-open" } else { "" })
            aria-hidden=move || (!open.get()).to_string()
            inert=move || (!open.get()).then_some("")
            on:click=move |_| open.set(false)
        >
            <div
                class=format!("dialog {}", class)
                role="dialog"
                aria-modal="true"
                aria-label=title_label
                on:click=|event| event.stop_propagation()
            >
                <header class="dialog__head">
                    <span class="eyebrow">"// 控制台 / 输入"</span>
                    <h2 class="dialog__title">{title}</h2>
                    <Show when=move || has_description>
                        <p class="dialog__desc">{description.clone()}</p>
                    </Show>
                </header>
                <div class="dialog__body">{children()}</div>
            </div>
        </div>
    }
}
