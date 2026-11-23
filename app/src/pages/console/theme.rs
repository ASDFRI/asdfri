use api::site::{SiteVariant, UpdateSiteVariant, get_site_variant};
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_meta::Title;
use ui::{Button, ButtonVariant, Panel, SnackbarState, Spinner, Tag, ToastKind};

use super::AuthSignal;
use crate::site::{SiteVariantSignal, variant_label};

/// 站点风格配置页
#[component]
pub fn ThemeAdmin() -> impl IntoView {
    let site_variant = SiteVariantSignal::use_state();
    let action = ServerAction::<UpdateSiteVariant>::new();
    let pending = Signal::derive(move || action.pending().get());
    let requested = RwSignal::new(None::<SiteVariant>);
    let is_authenticated = use_context::<AuthSignal>().unwrap_or_else(|| Signal::derive(|| false));
    let snackbar = SnackbarState::use_state();

    let sync_variant = move || {
        spawn_local(async move {
            if let Ok(value) = get_site_variant().await
                && requested.get_untracked().is_none()
                && site_variant.get_untracked() != value
            {
                site_variant.set(value);
            }
        });
    };

    // 1) 进入页面时读取数据库中的当前风格
    Effect::new(move |_| sync_variant());

    // 2) 保存成功后立即应用，避免回读旧值造成闪烁
    Effect::new(move || {
        if let Some(result) = action.value().get() {
            match result {
                Ok(()) => {
                    if let Some(value) = requested.get_untracked() {
                        site_variant.set(value);
                    }
                    snackbar.show("站点风格已更新，新请求将使用该风格。", ToastKind::Success);
                }
                Err(error) => snackbar.show(format!("风格更新失败：{error}"), ToastKind::Error),
            }
            action.value().set(None);
        }
    });

    let switch = Callback::new(move |variant: SiteVariant| {
        if pending.get_untracked() || site_variant.get() == variant {
            return;
        }

        requested.set(Some(variant));
        action.dispatch(UpdateSiteVariant {
            variant,
        });
    });

    view! {
        <Title text="Theme — ASDF Console"/>
        <header class="console-page-head">
            <div>
                <span class="eyebrow">"Visual System"</span>
                <h1 class="console-page-title">"站点风格"</h1>
                <p class="console-page-lede">
                    "控制公开站点与控制台的视觉体系。设置持久化到数据库，下一次服务端渲染立即生效。"
                </p>
            </div>
            <span class="mono console-page-code">"05 / THEME"</span>
        </header>

        <Show when=move || !is_authenticated.get()>
            <div class="lock-panel">
                <span class="mono">"READ ONLY"</span>
                <p>"当前会话未认证：可以查看当前风格，但切换会被服务端拒绝。"</p>
            </div>
        </Show>

        <div class="admin-grid admin-grid--pair">
            {move || {
                let active = site_variant.get();
                SiteVariant::ALL
                    .into_iter()
                    .enumerate()
                    .map(|(index, variant)| {
                        let selected = variant == active;
                        view! {
                            <Panel dark=selected class="admin-panel theme-option">
                                <span class="eyebrow">
                                    {format!("0{} / {}", index + 1, variant.as_str().to_uppercase())}
                                </span>
                                <h2 class="admin-panel__title">{variant_label(variant)}</h2>
                                <p class="admin-panel__text">{variant_description(variant)}</p>
                                <div class="tag-row">
                                    {variant_tags(variant)
                                        .into_iter()
                                        .map(|label| view! { <Tag label=label/> })
                                        .collect_view()}
                                </div>
                                <Button
                                    variant=if selected {
                                        ButtonVariant::Ghost
                                    } else {
                                        ButtonVariant::Solid
                                    }
                                    disabled=Signal::derive(move || {
                                        !is_authenticated.get()
                                            || pending.get()
                                            || site_variant.get() == variant
                                    })
                                    on_click=Callback::new(move |_| switch.run(variant))
                                >
                                    <Show when=move || {
                                        pending.get() && requested.get() == Some(variant)
                                    }>
                                        <Spinner class="spinner--sm"/>
                                    </Show>
                                    {if selected { "当前风格" } else { "切换到此风格" }}
                                </Button>
                            </Panel>
                        }
                    })
                    .collect_view()
            }}
        </div>
    }
}

/// 风格说明
fn variant_description(variant: SiteVariant) -> &'static str {
    match variant {
        SiteVariant::Swiss => {
            "严格网格、克制线条与明确层级；纸白、炭黑与单一正红构成可检验的秩序。"
        }
        SiteVariant::Constructivist => {
            "不对称网格、斜切色块、粗描边与硬阴影；米白、炭黑与信号红组织宣传海报式版面。"
        }
    }
}

/// 风格特征标签
fn variant_tags(variant: SiteVariant) -> [&'static str; 3] {
    match variant {
        SiteVariant::Swiss => ["GRID", "WHITE SPACE", "SWISS"],
        SiteVariant::Constructivist => ["SKEW", "HARD SHADOW", "THICK BORDER"],
    }
}
