use leptos::prelude::*;
use leptos_meta::{Meta, Title};
use leptos_router::components::A;

/// 路由未命中页面
#[component]
pub fn NotFound() -> impl IntoView {
    // 可用分区导航：地址、编号与名称
    const SECTORS: [(&str, &str, &str); 4] = [
        ("/technology", "01", "Technology"),
        ("/trailer", "02", "Trailer"),
        ("/games", "03", "Games"),
        ("/singularis", "04", "Singularis"),
    ];

    view! {
        <Title text="404 — ASDF Studio"/>
        <Meta name="robots" content="noindex"/>
        <section class="not-found first-screen first-screen--center">
            <div class="shell not-found__grid">
                <span class="not-found__glyph display display__accent">"404"</span>
                <div class="not-found__body">
                    <span class="eyebrow">"Out of Grid"</span>
                    <h1 class="display">"页面不存在"</h1>
                    <p class="lede">
                        "你访问的坐标不在当前网格之内。请返回索引，或从下方分区重新选择条目。"
                    </p>
                    <a href="/" class="btn btn--solid">"返回首页"</a>
                </div>
                <nav class="not-found__sectors mono" aria-label="可用分区">
                    {SECTORS
                        .iter()
                        .map(|(href, index, label)| {
                            view! {
                                <A href=*href attr:class="not-found__sector">
                                    <span class="not-found__sector-index">{*index}</span>
                                    <span>{*label}</span>
                                </A>
                            }
                        })
                        .collect_view()}
                </nav>
            </div>
        </section>
    }
}
