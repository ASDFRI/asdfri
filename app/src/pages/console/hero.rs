use leptos::prelude::*;
use leptos_meta::Title;
use ui::{Panel, Tag};

use super::AuthSignal;

/// Hero 内容槽位
struct Slot {
    key: &'static str,
    label: &'static str,
    value: &'static str,
    rule: &'static str,
}

const SLOTS: [Slot; 5] = [
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
        rule: "三行断句，强调色仅用于第二行",
    },
    Slot {
        key: "LEDE",
        label: "引导文本",
        value: "结构主义几何秩序与瑞士国际主义网格理性",
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
];

/// Hero 配置说明
#[component]
pub fn HeroConfig() -> impl IntoView {
    let is_authenticated = use_context::<AuthSignal>().unwrap_or_else(|| Signal::derive(|| false));

    view! {
        <Title text="Hero — ASDF Console"/>
        <header class="console-page-head">
            <div>
                <span class="eyebrow">"Composition"</span>
                <h1 class="console-page-title">"Hero 配置"</h1>
                <p class="console-page-lede">
                    "首页首屏的内容槽位与排版约束。当前版本将文案视为设计系统的一部分，修改需同步 SCSS 断点。"
                </p>
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
            {SLOTS
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
                .collect_view()}
        </div>

        <div class="admin-grid">
            <Panel class="admin-panel">
                <span class="eyebrow">"Grid"</span>
                <h2 class="admin-panel__title">"12 列网格"</h2>
                <p class="admin-panel__text">
                    "桌面端 12 列，间距随视口在 24–48px 之间变化；移动端折叠为单列，保留编号与分隔线。"
                </p>
                <div class="tag-row">
                    <Tag label="12 COL"/>
                    <Tag label="GUTTER 24–48"/>
                    <Tag label="MAX 1440"/>
                </div>
            </Panel>

            <Panel dark=true class="admin-panel">
                <span class="eyebrow">"Contrast"</span>
                <h2 class="admin-panel__title">"色彩约束"</h2>
                <p class="admin-panel__text">
                    "纸白、炭黑与单一正红。强调色只用于主操作与关键编号，不用于装饰性背景。"
                </p>
                <div class="tag-row">
                    <Tag label="PAPER"/>
                    <Tag label="INK"/>
                    <Tag label="SIGNAL RED"/>
                </div>
            </Panel>
        </div>
    }
}
