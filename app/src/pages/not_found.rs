use leptos::prelude::*;
use leptos_meta::{Meta, Title};

/// 路由未命中页面
#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <Title text="404 — ASDF Studio"/>
        <Meta name="robots" content="noindex"/>
        <section class="not-found">
            <div class="shell not-found__grid">
                <span class="not-found__code mono">"404"</span>
                <div>
                    <span class="eyebrow">"Out of Grid"</span>
                    <h1 class="display">"页面不存在"</h1>
                    <p class="lede">
                        "你访问的坐标不在当前网格之内。请返回索引，或从主导航重新选择条目。"
                    </p>
                    <a href="/" class="btn btn--solid">"返回首页"</a>
                </div>
            </div>
        </section>
    }
}
