use leptos::prelude::*;
use leptos_router::components::{A, Outlet};
use ui::Tag;

/// 站内导航条目
fn nav_items() -> &'static [(&'static str, &'static str, &'static str)] {
    &[
        ("/technology", "Technology", "01"),
        ("/trailer", "Trailer", "02"),
        ("/games", "Games", "03"),
        ("/singularis", "Singularis", "04"),
    ]
}

/// 公开站点布局：页头、路由出口与页脚
#[component]
pub fn SiteLayout() -> impl IntoView {
    view! {
        <SiteHeader/>
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
    view! {
        <footer class="site-footer">
            <div class="shell site-footer__grid">
                <div class="site-footer__brand">
                    <span class="eyebrow">"ASDF Studio"</span>
                    <p class="site-footer__statement">
                        "以严格的网格、克制的线条与明确的层级，构建可被检验的交互体验。"
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
