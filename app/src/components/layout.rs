use api::site::SiteVariant;
use leptos::prelude::*;
use leptos_router::components::{A, Outlet};
use ui::Tag;

use crate::site::SiteVariantSignal;

/// 站内导航条目
fn nav_items() -> &'static [(&'static str, &'static str, &'static str)] {
    &[
        ("/technology", "Technology", "01"),
        ("/trailer", "Trailer", "02"),
        ("/games", "Games", "03"),
        ("/singularis", "Singularis", "04"),
    ]
}

/// 纯构成主义宣传条文本
const AGITATION: [&str; 5] =
    ["构成主义", "CONSTRUCTIVISM", "几何 · 秩序 · 生产", "NO DECORATION", "ASDF / 2026"];

/// 宣传条内容组
#[component]
fn AgitationGroup() -> impl IntoView {
    view! {
        <div class="agitation-strip__group">
            {AGITATION
                .into_iter()
                .map(|text| view! { <span>{text}</span> })
                .collect_view()}
        </div>
    }
}

/// 纯构成主义宣传条
#[component]
fn AgitationStrip() -> impl IntoView {
    view! {
        <div class="agitation-strip" aria-hidden="true">
            <div class="agitation-strip__track">
                <AgitationGroup/>
                <AgitationGroup/>
            </div>
        </div>
    }
}

/// 史诗叙事序章条文本
const OVERTURE: [&str; 5] =
    ["第一章", "EPIC NARRATIVE", "神话 · 远征 · 黎明", "THE SAGA BEGINS", "ASDF / 2026"];

/// 史诗叙事序章条内容组
#[component]
fn OvertureGroup() -> impl IntoView {
    view! {
        <div class="epic-overture__group">
            {OVERTURE
                .into_iter()
                .map(|text| view! { <span>{text}</span> })
                .collect_view()}
        </div>
    }
}

/// 史诗叙事序章条
#[component]
fn EpicOverture() -> impl IntoView {
    view! {
        <div class="epic-overture" aria-hidden="true">
            <div class="epic-overture__track">
                <OvertureGroup/>
                <OvertureGroup/>
            </div>
        </div>
    }
}

/// 公开站点布局：页头、风格化宣传条、路由出口与页脚
#[component]
pub fn SiteLayout() -> impl IntoView {
    let variant = SiteVariantSignal::use_state();

    view! {
        <SiteHeader/>
        {move || {
            (variant.get() == SiteVariant::Constructivist).then(|| view! { <AgitationStrip/> })
        }}
        {move || (variant.get() == SiteVariant::Epic).then(|| view! { <EpicOverture/> })}
        <main id="main"><Outlet/></main>
        <SiteFooter/>
    }
}

/// 全局页头
#[component]
pub fn SiteHeader() -> impl IntoView {
    let menu_open = RwSignal::new(false);

    view! {
        <header class="site-header">
            <div class="shell site-header__inner">
                <A
                    href="/"
                    exact=true
                    attr:class="brand"
                    on:click=move |_| menu_open.set(false)
                >
                    <svg class="brand__mark" viewBox="0 0 48 48" aria-hidden="true">
                        <path d="M4 42L20 6h8l16 36h-9l-3-8H16l-3 8H4z" fill="none" stroke="currentColor" stroke-width="2.5"/>
                        <path d="M19 27h10" stroke="currentColor" stroke-width="2.5"/>
                    </svg>
                    <span class="brand__text">
                        <strong>"ASDF"</strong>
                        <span>"STUDIO"</span>
                    </span>
                </A>

                <nav class="nav-desktop" aria-label="主导航">
                    {nav_items()
                        .iter()
                        .map(|(href, label, index)| {
                            view! {
                                <A href=*href attr:class="nav-link">
                                    <span class="nav-link__index">{*index}</span>
                                    <span>{*label}</span>
                                </A>
                            }
                        })
                        .collect_view()}
                </nav>

                <div class="site-header__actions">
                    <A href="/console" attr:class="btn btn--outline btn--sm">
                        "控制台"
                    </A>
                    <div class="nav-mobile">
                        <button
                            class="nav-mobile__toggle"
                            on:click=move |_| menu_open.update(|open| *open = !*open)
                            aria-expanded=move || menu_open.get().to_string()
                        >
                            <span>"菜单"</span>
                            <span class="nav-mobile__bars" aria-hidden="true">
                                <i></i>
                                <i></i>
                                <i></i>
                            </span>
                        </button>
                        <Show when=move || menu_open.get()>
                            <div class="nav-mobile__panel">
                                {nav_items()
                                    .iter()
                                    .map(|(href, label, index)| {
                                        view! {
                                            <A
                                                href=*href
                                                attr:class="nav-mobile__link"
                                                on:click=move |_| menu_open.set(false)
                                            >
                                                <span class="nav-link__index">{*index}</span>
                                                <span>{*label}</span>
                                            </A>
                                        }
                                    })
                                    .collect_view()}
                                <A
                                    href="/console"
                                    attr:class="nav-mobile__link"
                                    on:click=move |_| menu_open.set(false)
                                >
                                    <span class="nav-link__index">"05"</span>
                                    <span>"控制台"</span>
                                </A>
                            </div>
                        </Show>
                    </div>
                </div>
            </div>
        </header>
    }
}

/// 全局页脚
#[component]
pub fn SiteFooter() -> impl IntoView {
    let variant = SiteVariantSignal::use_state();

    view! {
        <footer class="site-footer">
            {move || {
                (variant.get() == SiteVariant::Constructivist)
                    .then(|| {
                        view! {
                            <div class="shell footer-manifesto">
                                <div class="footer-manifesto__text">
                                    <span class="mono footer-manifesto__index">"// MANIFESTO"</span>
                                    <p>"不装饰世界。组织世界。"</p>
                                </div>
                                <span class="mono footer-manifesto__en">
                                    "DO NOT DECORATE. ORGANIZE."
                                </span>
                            </div>
                        }
                    })
            }}

            {move || {
                (variant.get() == SiteVariant::Epic)
                    .then(|| {
                        view! {
                            <div class="shell footer-epic">
                                <div class="footer-epic__text">
                                    <span class="mono footer-epic__index">"// THE SAGA"</span>
                                    <p>"每一个世界，都始于一次不甘平凡的启程。"</p>
                                </div>
                                <span class="mono footer-epic__en">
                                    "EVERY WORLD BEGINS WITH A DEPARTURE."
                                </span>
                            </div>
                        }
                    })
            }}

            <div class="shell site-footer__grid">
                <div class="site-footer__brand">
                    <span class="eyebrow">"ASDF Studio"</span>
                    <p class="site-footer__statement">
                        {move || match variant.get() {
                            SiteVariant::Swiss => {
                                "以严格的网格、克制的线条与明确的层级，构建可被检验的交互体验。"
                            }
                            SiteVariant::Constructivist => {
                                "红、黑、米白。用几何、色块与生产力组织世界，而不是装饰世界。"
                            }
                            SiteVariant::Epic => {
                                "深空黑、鎏金与长夜。把每一次交互写成可以传颂的远征叙事。"
                            }
                        }}
                    </p>
                </div>

                <div class="site-footer__col">
                    <span class="eyebrow">"目录"</span>
                    <A href="/technology" attr:class="footer-link">"Technology"</A>
                    <A href="/trailer" attr:class="footer-link">"Trailer"</A>
                    <A href="/games" attr:class="footer-link">"Games"</A>
                    <A href="/singularis" attr:class="footer-link">"Singularis"</A>
                </div>

                <div class="site-footer__col">
                    <span class="eyebrow">"技术栈"</span>
                    <div class="tag-row">
                        <Tag label="Rust"/>
                        <Tag label="Leptos"/>
                        <Tag label="Axum"/>
                        <Tag label="SQLite"/>
                        <Tag label="WebAssembly"/>
                    </div>
                </div>

                <div class="site-footer__col">
                    <span class="eyebrow">"联络"</span>
                    <a href="mailto:asdf@gmail.com" class="footer-link">"asdf@gmail.com"</a>
                    <a href="https://www.bilibili.com/video/BV1U6qDBmEyz/" target="_blank" rel="noopener noreferrer" class="footer-link">
                        "Bilibili"
                    </a>
                    <a href="#" class="footer-link">"Twitter / X"</a>
                    <a href="#" class="footer-link">"Discord"</a>
                    <a href="#" class="footer-link">"YouTube"</a>
                </div>
            </div>

            <div class="shell site-footer__base">
                <span>"© 2026 ASDF STUDIO. All rights reserved."</span>
                <nav class="site-footer__legal" aria-label="法律信息">
                    <a href="#">"服务条款"</a>
                    <a href="#">"隐私政策"</a>
                    <a href="#">"Cookie 政策"</a>
                </nav>
                <span class="mono">"DESIGNED & BUILT BY TRIFINGZW"</span>
            </div>
        </footer>
    }
}
