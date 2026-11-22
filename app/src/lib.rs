#![recursion_limit = "1024"]

//! 应用层：装配页面、元信息与路由。

mod components;
mod pages;

use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, provide_meta_context};
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    path,
};

use crate::{
    components::SiteLayout,
    pages::{
        Console, Dashboard, GameAdmin, GamesPage, HeroConfig, HomePage, MemberAdmin, NotFound,
        SingularisPage, TechnologyPage, TrailerPage,
    },
};

/// HTML 文档骨架，由服务端在 SSR 时渲染
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="zh-CN" data-variant="constructivist">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <meta name="color-scheme" content="light"/>
                <meta name="theme-color" content="#0b0b0b"/>
                <link rel="icon" href="/favicon.ico"/>
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

    view! {
        <Stylesheet id="leptos" href="/pkg/asdfri.css"/>

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
                </ParentRoute>
            </Routes>
        </Router>
    }
}
