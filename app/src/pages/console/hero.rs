use api::site::SiteVariant;
use leptos::prelude::*;
use leptos_meta::Title;
use ui::{Panel, Tag};

use super::AuthSignal;
use crate::site::SiteVariantSignal;

/// Hero 内容槽位
struct Slot {
    key: &'static str,
    label: &'static str,
    value: &'static str,
    rule: &'static str,
}

/// 面板说明
#[derive(Clone, Copy)]
struct PanelCopy {
    eyebrow: &'static str,
    title: &'static str,
    text: &'static str,
    tags: [&'static str; 3],
}

/// 当前风格的 Hero 说明
#[derive(Clone, Copy)]
struct HeroCopy {
    lede: &'static str,
    geometry: PanelCopy,
    contrast: PanelCopy,
}

/// 当前风格的内容槽位
fn slots(variant: SiteVariant) -> [Slot; 5] {
    let (title_rule, lede_value) = match variant {
        SiteVariant::Swiss => {
            ("三行断句，强调色仅用于第二行", "结构主义几何秩序与瑞士国际主义网格理性")
        }
        SiteVariant::Constructivist => {
            ("三行断句，第二行使用红色色块", "构成主义几何秩序、斜切色块与生产力")
        }
    };

    [
        Slot {
            key: "EYEBROW",
            label: "眉题",
            value: "ASDF Studio / 2026",
            rule: "等宽大写，字距 0.22em",
        },
        Slot {
            key: "TITLE",
            label: "主标题",
            value: "我们创造 / 虚拟 / 世界",
            rule: title_rule,
        },
        Slot {
            key: "LEDE",
            label: "引导文本",
            value: lede_value,
            rule: "不超过两行，宽度不超过 60ch",
        },
        Slot {
            key: "PRIMARY",
            label: "主操作",
            value: "查看作品 → #games",
            rule: "实心按钮，唯一强调色",
        },
        Slot {
            key: "SECONDARY",
            label: "次操作",
            value: "发起合作 → #contact",
            rule: "线框按钮，不竞争视觉焦点",
        },
    ]
}

/// 当前风格的说明文案
fn copy_for(variant: SiteVariant) -> HeroCopy {
    match variant {
        SiteVariant::Swiss => HeroCopy {
            lede: "首页首屏的内容槽位与排版约束。当前版本将文案视为设计系统的一部分，修改需同步 SCSS 断点。",
            geometry: PanelCopy {
                eyebrow: "Grid",
                title: "12 列网格",
                text: "桌面端 12 列，间距随视口在 24–48px 之间变化；移动端折叠为单列，保留编号与分隔线。",
                tags: ["12 COL", "GUTTER 24–48", "MAX 1440"],
            },
            contrast: PanelCopy {
                eyebrow: "Contrast",
                title: "色彩约束",
                text: "纸白、炭黑与单一正红。强调色只用于主操作与关键编号，不用于装饰性背景。",
                tags: ["PAPER", "INK", "SIGNAL RED"],
            },
        },
        SiteVariant::Constructivist => HeroCopy {
            lede: "纯构成主义风格的首页首屏内容槽位与排版约束。当前版本将文案视为设计系统的一部分，修改需同步 SCSS 断点。",
            geometry: PanelCopy {
                eyebrow: "Geometry",
                title: "斜切几何",
                text: "以不对称网格、斜切色块、粗描边与硬阴影构成页面；拒绝柔和圆角与低对比装饰。",
                tags: ["SKEW", "HARD SHADOW", "THICK BORDER"],
            },
            contrast: PanelCopy {
                eyebrow: "Contrast",
                title: "三色限制",
                text: "仅使用米白、炭黑与信号红三色。红色用于宣传条、关键编号、主操作与大面积切角。",
                tags: ["PAPER", "INK", "SIGNAL RED"],
            },
        },
    }
}

/// Hero 配置说明
#[component]
pub fn HeroConfig() -> impl IntoView {
    let variant = SiteVariantSignal::use_state();
    let is_authenticated = use_context::<AuthSignal>().unwrap_or_else(|| Signal::derive(|| false));
    let geometry = move || copy_for(variant.get()).geometry;
    let contrast = move || copy_for(variant.get()).contrast;

    view! {
        <Title text="Hero — ASDF Console"/>
        <header class="console-page-head">
            <div>
                <span class="eyebrow">"Composition"</span>
                <h1 class="console-page-title">"Hero 配置"</h1>
                <p class="console-page-lede">{move || copy_for(variant.get()).lede}</p>
            </div>
            <span class="mono console-page-code">"02 / HERO"</span>
        </header>

        <Show when=move || !is_authenticated.get()>
            <div class="lock-panel">
                <span class="mono">"READ ONLY"</span>
                <p>"未认证会话仅可查看当前首屏规格。"</p>
            </div>
        </Show>

        <div class="slot-list">
            {move || {
                slots(variant.get())
                    .into_iter()
                    .map(|slot| {
                        view! {
                            <article class="slot">
                                <span class="mono slot__key">{slot.key}</span>
                                <div class="slot__content">
                                    <span class="slot__label">{slot.label}</span>
                                    <p class="slot__value">{slot.value}</p>
                                </div>
                                <span class="mono slot__rule">{slot.rule}</span>
                            </article>
                        }
                    })
                    .collect_view()
            }}
        </div>

        <div class="admin-grid">
            <Panel class="admin-panel">
                <span class="eyebrow">{move || geometry().eyebrow}</span>
                <h2 class="admin-panel__title">{move || geometry().title}</h2>
                <p class="admin-panel__text">{move || geometry().text}</p>
                <div class="tag-row">
                    {move || {
                        geometry()
                            .tags
                            .into_iter()
                            .map(|label| view! { <Tag label=label/> })
                            .collect_view()
                    }}
                </div>
            </Panel>

            <Panel dark=true class="admin-panel">
                <span class="eyebrow">{move || contrast().eyebrow}</span>
                <h2 class="admin-panel__title">{move || contrast().title}</h2>
                <p class="admin-panel__text">{move || contrast().text}</p>
                <div class="tag-row">
                    {move || {
                        contrast()
                            .tags
                            .into_iter()
                            .map(|label| view! { <Tag label=label/> })
                            .collect_view()
                    }}
                </div>
            </Panel>
        </div>
    }
}
