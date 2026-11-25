use api::{game::get_games, member::get_members};
use leptos::prelude::*;
use leptos_meta::Title;
use ui::{Panel, Skeleton, Tag};

use super::AuthSignal;

/// 控制台仪表盘
#[component]
pub fn Dashboard() -> impl IntoView {
    let games = Resource::new(|| (), |_| get_games());
    let members = Resource::new(|| (), |_| get_members());
    let is_authenticated = use_context::<AuthSignal>().unwrap_or_else(|| Signal::derive(|| false));

    view! {
        <Title text="Dashboard — ASDF Console"/>
        <header class="console-page-head">
            <div>
                <span class="eyebrow">"Overview"</span>
                <h1 class="console-page-title">"仪表盘"</h1>
                <p class="console-page-lede">
                    "内容库的实时概览。所有统计直接来自数据库，不使用缓存数值。"
                </p>
            </div>
            <span class="mono console-page-code">"01 / DASH"</span>
        </header>

        <Show when=move || !is_authenticated.get()>
            <div class="lock-panel">
                <span class="mono">"LOCKED"</span>
                <p>"当前会话未认证：可以浏览数据，但无法执行写入操作。"</p>
            </div>
        </Show>

        <div class="stat-grid stat-grid--console">
            <Suspense fallback=move || view! { <Skeleton class="stat stat--skeleton"/> }>
                {move || {
                    games
                        .get()
                        .map(|result| match result {
                            Ok(games) => {
                                view! {
                                    <div class="stat">
                                        <span class="stat__value">{games.len().to_string()}</span>
                                        <span class="stat__label">"游戏档案"</span>
                                    </div>
                                }
                                    .into_any()
                            }
                            Err(error) => {
                                view! {
                                    <div class="stat stat--error">
                                        <span class="stat__label">"读取失败"</span>
                                        <span class="mono">{error.to_string()}</span>
                                    </div>
                                }
                                    .into_any()
                            }
                        })
                }}
            </Suspense>

            <Suspense fallback=move || view! { <Skeleton class="stat stat--skeleton"/> }>
                {move || {
                    members
                        .get()
                        .map(|result| match result {
                            Ok(members) => {
                                view! {
                                    <div class="stat">
                                        <span class="stat__value">{members.len().to_string()}</span>
                                        <span class="stat__label">"成员档案"</span>
                                    </div>
                                }
                                    .into_any()
                            }
                            Err(error) => {
                                view! {
                                    <div class="stat stat--error">
                                        <span class="stat__label">"读取失败"</span>
                                        <span class="mono">{error.to_string()}</span>
                                    </div>
                                }
                                    .into_any()
                            }
                        })
                }}
            </Suspense>

            <div class="stat">
                <span class="stat__value">"RUST"</span>
                <span class="stat__label">"运行时"</span>
            </div>
            <div class="stat">
                <span class="stat__value">"SQLite"</span>
                <span class="stat__label">"持久层"</span>
            </div>
        </div>

        <div class="admin-grid admin-grid--pair">
            <Panel class="admin-panel">
                <span class="eyebrow">"Games"</span>
                <h2 class="admin-panel__title">"游戏档案"</h2>
                <p class="admin-panel__text">
                    "维护作品元数据、截图资产与展示顺序。写入操作需要有效会话。"
                </p>
                <a href="/console/games" class="btn btn--solid">"进入游戏配置"</a>
            </Panel>

            <Panel class="admin-panel">
                <span class="eyebrow">"Members"</span>
                <h2 class="admin-panel__title">"成员档案"</h2>
                <p class="admin-panel__text">
                    "维护团队成员的身份信息、头像与展示顺序。"
                </p>
                <a href="/console/members" class="btn btn--solid">"进入成员配置"</a>
            </Panel>

            <Panel class="admin-panel">
                <span class="eyebrow">"Visual System"</span>
                <h2 class="admin-panel__title">"站点风格"</h2>
                <p class="admin-panel__text">
                    "在瑞士国际主义、纯构成主义与史诗叙事之间切换；设置持久化到数据库并作用于下一次渲染。"
                </p>
                <a href="/console/theme" class="btn btn--solid">"进入风格配置"</a>
            </Panel>

            <Panel dark=true class="admin-panel">
                <span class="eyebrow">"System"</span>
                <h2 class="admin-panel__title">"操作约束"</h2>
                <div class="tag-row">
                    <Tag label="JWT"/>
                    <Tag label="HttpOnly"/>
                    <Tag label="SameSite=Lax"/>
                    <Tag label="SQLite"/>
                </div>
                <p class="admin-panel__text">
                    "所有写操作在服务端再次校验令牌；客户端状态不构成授权依据。"
                </p>
            </Panel>
        </div>
    }
}
