use api::site::{HeroContent, HeroPayload, SiteVariant, UpdateHero, get_hero};
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_meta::Title;
use ui::{Button, ButtonVariant, Field, Panel, SnackbarState, Spinner, Tag, Textarea, ToastKind};

use super::AuthSignal;
use crate::site::{SiteVariantSignal, variant_label};

/// 面板说明
#[derive(Clone, Copy)]
struct PanelCopy {
    eyebrow: &'static str,
    title: &'static str,
    text: &'static str,
    tags: [&'static str; 3],
}

/// 当前风格的 Hero 设计说明
#[derive(Clone, Copy)]
struct HeroNotes {
    geometry: PanelCopy,
    contrast: PanelCopy,
}

/// 当前风格的设计说明
fn notes_for(variant: SiteVariant) -> HeroNotes {
    match variant {
        SiteVariant::Swiss => HeroNotes {
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
        SiteVariant::Constructivist => HeroNotes {
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
        SiteVariant::Epic => HeroNotes {
            geometry: PanelCopy {
                eyebrow: "Composition",
                title: "章节构图",
                text: "以电影画幅、超大衬线标题与仪式化留白组织页面；鎏金细线划分章节，光晕聚焦视线。",
                tags: ["CINEMA", "SERIF", "GOLD RULE"],
            },
            contrast: PanelCopy {
                eyebrow: "Contrast",
                title: "深渊与鎏金",
                text: "深空黑承载主体，鎏金只用于标题、编号与主操作；图像统一为暖调低饱和，避免彩色噪声。",
                tags: ["MIDNIGHT", "GOLD LEAF", "WARM GRADE"],
            },
        },
    }
}

/// Hero 配置页
#[component]
pub fn HeroConfig() -> impl IntoView {
    let selected = RwSignal::new(SiteVariantSignal::use_state().get());
    let action = ServerAction::<UpdateHero>::new();
    let pending = Signal::derive(move || action.pending().get());
    let snackbar = SnackbarState::use_state();
    let is_authenticated = use_context::<AuthSignal>().unwrap_or_else(|| Signal::derive(|| false));

    let eyebrow = RwSignal::new(String::new());
    let location = RwSignal::new(String::new());
    let title_text = RwSignal::new(String::new());
    let lede = RwSignal::new(String::new());
    let primary_label = RwSignal::new(String::new());
    let primary_href = RwSignal::new(String::new());
    let secondary_label = RwSignal::new(String::new());
    let secondary_href = RwSignal::new(String::new());
    let media_tag = RwSignal::new(String::new());
    let media_poster = RwSignal::new(String::new());
    let media_video = RwSignal::new(String::new());
    let strip_text = RwSignal::new(String::new());
    let baseline = RwSignal::new(None::<HeroContent>);
    let loading = RwSignal::new(true);
    let load_error = RwSignal::new(None::<String>);

    let apply_content = move |content: HeroContent| {
        eyebrow.set(content.eyebrow);
        location.set(content.location);
        title_text.set(content.title.join("\n"));
        lede.set(content.lede);
        primary_label.set(content.primary_label);
        primary_href.set(content.primary_href);
        secondary_label.set(content.secondary_label);
        secondary_href.set(content.secondary_href);
        media_tag.set(content.media_tag);
        media_poster.set(content.media_poster);
        media_video.set(content.media_video);
        strip_text.set(content.strip.join(", "));
    };

    // 1) 立即以默认内容填充表单，避免页面切换时出现骨架闪烁
    apply_content(HeroContent::defaults(selected.get_untracked()));

    let request = move |variant: SiteVariant| {
        loading.set(true);
        load_error.set(None);
        spawn_local(async move {
            match get_hero(variant).await {
                Ok(content) => {
                    apply_content(content.clone());
                    baseline.set(Some(content));
                    load_error.set(None);
                }
                Err(error) => load_error.set(Some(error.to_string())),
            }
            loading.set(false);
        });
    };

    let form_content = move || {
        let title = title_text
            .get()
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(str::to_string)
            .collect::<Vec<_>>();

        let mut strip = Vec::new();
        for label in strip_text.get().split([',', '，']) {
            let label = label.trim();
            if label.is_empty() || strip.iter().any(|item: &String| item.as_str() == label) {
                continue;
            }
            strip.push(label.to_string());
            if strip.len() == 8 {
                break;
            }
        }

        HeroContent {
            eyebrow: eyebrow.get().trim().to_string(),
            location: location.get().trim().to_string(),
            title,
            lede: lede.get().trim().to_string(),
            primary_label: primary_label.get().trim().to_string(),
            primary_href: primary_href.get().trim().to_string(),
            secondary_label: secondary_label.get().trim().to_string(),
            secondary_href: secondary_href.get().trim().to_string(),
            media_tag: media_tag.get().trim().to_string(),
            media_poster: media_poster.get().trim().to_string(),
            media_video: media_video.get().trim().to_string(),
            strip,
        }
    };

    let dirty = Signal::derive(move || baseline.get().is_some_and(|base| base != form_content()));
    let controls_disabled =
        Signal::derive(move || !is_authenticated.get() || pending.get() || loading.get());
    let edit_disabled = Signal::derive(move || {
        controls_disabled.get() || !dirty.get() || loading.get() || load_error.get().is_some()
    });
    let switch_disabled = Signal::derive(move || pending.get() || loading.get());
    let notes = move || notes_for(selected.get());

    // 2) 进入页面或切换变体时读取已保存的配置
    Effect::new(move |_| {
        request(selected.get());
    });

    // 3) 保存成功后以本地规范值为新基线
    Effect::new(move || {
        if let Some(result) = action.value().get() {
            match result {
                Ok(()) => {
                    let content = untrack(form_content);
                    apply_content(content.clone());
                    baseline.set(Some(content));
                    snackbar.show("Hero 配置已更新。", ToastKind::Success);
                }
                Err(error) => snackbar.show(format!("更新失败：{error}"), ToastKind::Error),
            }
            action.value().set(None);
        }
    });

    let on_variant = Callback::new(move |variant: SiteVariant| {
        if variant == selected.get_untracked() {
            return;
        }
        if dirty.get_untracked() {
            snackbar.show("存在未保存的修改，请先保存或重置。", ToastKind::Error);
            return;
        }

        load_error.set(None);
        baseline.set(None);
        loading.set(true);
        apply_content(HeroContent::defaults(variant));
        selected.set(variant);
    });

    let on_reset = Callback::new(move |_: leptos::ev::MouseEvent| {
        let Some(content) = baseline.get_untracked() else {
            return;
        };
        apply_content(content);
        snackbar.show("已还原到上次保存的内容。", ToastKind::Info);
    });

    let submit = move || {
        if controls_disabled.get_untracked() || !dirty.get_untracked() {
            return;
        }

        let content = form_content();
        if content.eyebrow.is_empty() {
            snackbar.show("眉题不能为空。", ToastKind::Error);
            return;
        }
        if content.title.is_empty() {
            snackbar.show("主标题至少需要一行。", ToastKind::Error);
            return;
        }
        if content.title.len() > 3 {
            snackbar.show("主标题最多三行。", ToastKind::Error);
            return;
        }
        if content.lede.is_empty() {
            snackbar.show("引导文本不能为空。", ToastKind::Error);
            return;
        }
        if content.primary_label.is_empty() {
            snackbar.show("主操作文案不能为空。", ToastKind::Error);
            return;
        }
        if content.primary_href.is_empty() {
            snackbar.show("主操作链接不能为空。", ToastKind::Error);
            return;
        }
        if content.secondary_label.is_empty() != content.secondary_href.is_empty() {
            snackbar.show("次操作的文案与链接必须同时填写或同时留空。", ToastKind::Error);
            return;
        }

        action.dispatch(UpdateHero {
            variant: selected.get_untracked(),
            payload: HeroPayload::from(content),
        });
    };

    view! {
        <Title text="Hero — ASDF Console"/>
        <header class="console-page-head">
            <div>
                <span class="eyebrow">"Composition"</span>
                <h1 class="console-page-title">"Hero 配置"</h1>
                <p class="console-page-lede">
                    "编辑首页首屏的眉题、标题、导语、操作按钮与媒体槽位。内容按视觉变体分别保存，新的页面请求立即生效。"
                </p>
            </div>
            <span class="mono console-page-code">"02 / HERO"</span>
        </header>

        <Show when=move || !is_authenticated.get()>
            <div class="lock-panel">
                <span class="mono">"READ ONLY"</span>
                <p>"当前会话未认证：可以浏览全部变体的首屏配置，但保存会被服务端拒绝。"</p>
            </div>
        </Show>

        <div class="hero-toolbar">
            <span class="mono hero-toolbar__label">"EDITING VARIANT"</span>
            <div class="tag-row">
                {move || {
                    SiteVariant::ALL
                        .into_iter()
                        .map(|variant| {
                            let current = selected.get() == variant;
                            view! {
                                <Button
                                    variant=if current {
                                        ButtonVariant::Solid
                                    } else {
                                        ButtonVariant::Outline
                                    }
                                    disabled=switch_disabled
                                    on_click=Callback::new(move |_| on_variant.run(variant))
                                >
                                    {variant_label(variant)}
                                </Button>
                            }
                        })
                        .collect_view()
                }}
            </div>
            <Show when=move || dirty.get()>
                <span class="mono hero-toolbar__dirty">"UNSAVED"</span>
            </Show>
        </div>

        <Panel class="admin-panel hero-editor">
            <span class="eyebrow">"Hero Content"</span>
            <h2 class="admin-panel__title">{move || variant_label(selected.get())}</h2>
            <p class="admin-panel__text">
                {move || {
                    if loading.get() {
                        "正在读取已保存的配置…"
                    } else {
                        "内容按视觉变体独立保存；编辑另一变体不会改变当前站点风格。"
                    }
                }}
            </p>

            {move || {
                if let Some(error) = load_error.get() {
                    view! {
                        <div class="empty-state">
                            <span class="mono">"ERROR"</span>
                            <h2>"读取 Hero 配置失败"</h2>
                            <p>{error}</p>
                            <Button
                                variant=ButtonVariant::Outline
                                on_click=Callback::new(move |_| {
                                    request(selected.get_untracked());
                                })
                            >
                                "重试"
                            </Button>
                        </div>
                    }
                        .into_any()
                } else {
                    view! {
                        <form
                            class="form-stack"
                            on:submit=move |event| {
                                event.prevent_default();
                                submit();
                            }
                        >
                            <Field
                                label="眉题 · 等宽大写，字距 0.22em"
                                value=eyebrow
                                required=true
                                disabled=controls_disabled
                            />
                            <Field
                                label="地点标记 · 等宽大写，可留空"
                                value=location
                                disabled=controls_disabled
                            />
                            <Textarea
                                label="主标题 · 每行一句，最多三行，第二行使用强调色"
                                value=title_text
                                rows=3
                                required=true
                                disabled=controls_disabled
                            />
                            <Textarea
                                label="引导文本 · 建议不超过两行，60ch 以内"
                                value=lede
                                rows=3
                                required=true
                                disabled=controls_disabled
                            />
                            <div class="form-pair">
                                <Field
                                    label="主操作文案 · 实心按钮"
                                    value=primary_label
                                    required=true
                                    disabled=controls_disabled
                                />
                                <Field
                                    label="主操作链接 · 支持锚点、站内路径、mailto 或 http(s)"
                                    value=primary_href
                                    required=true
                                    disabled=controls_disabled
                                />
                            </div>
                            <div class="form-pair">
                                <Field
                                    label="次操作文案 · 线框按钮，与链接同时留空隐藏"
                                    value=secondary_label
                                    disabled=controls_disabled
                                />
                                <Field
                                    label="次操作链接"
                                    value=secondary_href
                                    disabled=controls_disabled
                                />
                            </div>
                            <Field
                                label="媒体角标 · 例如 // REEL 2026"
                                value=media_tag
                                disabled=controls_disabled
                            />
                            <div class="form-pair">
                                <Field
                                    label="视频封面路径 · 站内绝对路径或 http(s)，可留空"
                                    value=media_poster
                                    disabled=controls_disabled
                                />
                                <Field
                                    label="视频路径 · MP4，站内绝对路径或 http(s)，留空隐藏媒体"
                                    value=media_video
                                    disabled=controls_disabled
                                />
                            </div>
                            <Field
                                label="底部滚动标签 · 逗号分隔，可留空，最多 8 个"
                                value=strip_text
                                disabled=controls_disabled
                            />

                            <div class="dialog-actions">
                                <Button
                                    variant=ButtonVariant::Ghost
                                    disabled=edit_disabled
                                    on_click=on_reset
                                >
                                    "重置"
                                </Button>
                                <Button
                                    variant=ButtonVariant::Solid
                                    button_type="submit"
                                    disabled=edit_disabled
                                >
                                    <Show
                                        when=move || pending.get()
                                        fallback=move || view! { "保存更改" }
                                    >
                                        <Spinner class="spinner--sm"/>
                                        <span>"写入中"</span>
                                    </Show>
                                </Button>
                            </div>
                        </form>
                    }
                        .into_any()
                }
            }}
        </Panel>

        <div class="admin-grid admin-grid--pair">
            <Panel class="admin-panel">
                <span class="eyebrow">{move || notes().geometry.eyebrow}</span>
                <h2 class="admin-panel__title">{move || notes().geometry.title}</h2>
                <p class="admin-panel__text">{move || notes().geometry.text}</p>
                <div class="tag-row">
                    {move || {
                        notes()
                            .geometry
                            .tags
                            .into_iter()
                            .map(|label| view! { <Tag label=label/> })
                            .collect_view()
                    }}
                </div>
            </Panel>

            <Panel dark=true class="admin-panel">
                <span class="eyebrow">{move || notes().contrast.eyebrow}</span>
                <h2 class="admin-panel__title">{move || notes().contrast.title}</h2>
                <p class="admin-panel__text">{move || notes().contrast.text}</p>
                <div class="tag-row">
                    {move || {
                        notes()
                            .contrast
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
