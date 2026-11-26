#![recursion_limit = "1024"]

//! 应用层：装配页面、元信息与路由。

mod components;
mod pages;
mod site;

use leptos::prelude::*;
use leptos_meta::{HashedStylesheet, Meta, MetaTags, provide_meta_context};
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    path,
};

use crate::{
    components::SiteLayout,
    pages::{
        Console, Dashboard, GameAdmin, GamesPage, HeroConfig, HomePage, MemberAdmin, NotFound,
        SingularisPage, TechnologyPage, ThemeAdmin, TrailerPage,
    },
    site::{SiteVariantSignal, color_scheme, theme_color},
};

/// HTML 文档骨架，由服务端在 SSR 时渲染
pub fn shell(options: LeptosOptions) -> impl IntoView {
    let variant = use_context::<api::site::SiteVariant>().unwrap_or_default();

    view! {
        <!DOCTYPE html>
        <html lang="zh-CN" data-variant=variant.as_str()>
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <link rel="icon" href="/favicon.ico"/>
                <HashedStylesheet id="leptos" options=options.clone()/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

/// 应用根组件，装配元信息与路由
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let variant = SiteVariantSignal::provide();

    #[cfg(target_arch = "wasm32")]
    Effect::new(move || crate::site::sync_document_variant(variant.get()));

    view! {
        <Meta name="color-scheme" content=move || color_scheme(variant.get())/>
        <Meta name="theme-color" content=move || theme_color(variant.get())/>

        <Router>
            <Routes fallback=|| view! { <NotFound/> }>
                <ParentRoute path=path!("/") view=SiteLayout>
                    <Route path=path!("/") view=HomePage/>
                    <Route path=path!("/technology") view=TechnologyPage/>
                    <Route path=path!("/trailer") view=TrailerPage/>
                    <Route path=path!("/games") view=GamesPage/>
                    <Route path=path!("/singularis") view=SingularisPage/>
                </ParentRoute>

                <ParentRoute path=path!("/console") view=Console>
                    <Route path=path!("/") view=Dashboard/>
                    <Route path=path!("/hero") view=HeroConfig/>
                    <Route path=path!("/games") view=GameAdmin/>
                    <Route path=path!("/members") view=MemberAdmin/>
                    <Route path=path!("/theme") view=ThemeAdmin/>
                </ParentRoute>
            </Routes>
        </Router>
    }
}
