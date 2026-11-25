use leptos::prelude::*;
use leptos_meta::{Meta, Title};

use crate::components::SectionHead;

/// 预告片总时长（秒），时间轴标尺按它折算章节位置
const RUNTIME_SECS: u32 = 102;

/// 预告章节：起点秒数、时间码、标题与摘要
struct Chapter {
    at: u32,
    code: &'static str,
    title: &'static str,
    body: &'static str,
}

const CHAPTERS: [Chapter; 4] = [
    Chapter {
        at: 0,
        code: "00:00",
        title: "开场",
        body: "工作室标识与年度基调",
    },
    Chapter {
        at: 18,
        code: "00:18",
        title: "世界",
        body: "四个项目的片段交替",
    },
    Chapter {
        at: 54,
        code: "00:54",
        title: "系统",
        body: "引擎与工具链的快速剪辑",
    },
    Chapter {
        at: 84,
        code: "01:24",
        title: "署名",
        body: "团队与致谢",
    },
];

/// 预告片页
#[component]
pub fn TrailerPage() -> impl IntoView {
    const META: [(&str, &str); 5] = [
        ("FORMAT", "16:9 / 4K"),
        ("RUNTIME", "01:42"),
        ("LANGUAGE", "中文 / 英文字幕"),
        ("STATUS", "公开"),
        ("CHANNEL", "Bilibili"),
    ];

    view! {
        <Title text="Trailer — ASDF Studio"/>
        <Meta
            name="description"
            content="ASDF Studio 2026 年度预告片，于 Bilibili 观看。"
        />
        <section class="page-hero first-screen">
            <div class="shell trailer-hero__grid">
                <div class="trailer-hero__head">
                    <div class="trailer-hero__title">
                        <span class="index-mark">"03"</span>
                        <span class="eyebrow">"Screening Room"</span>
                        <h1 class="display">"年度预告"</h1>
                        <p class="lede">
                            "一次对四个世界的并行剪辑。没有旁白，只有系统运转的声音。"
                        </p>
                    </div>
                    <a
                        href="https://www.bilibili.com/video/BV1U6qDBmEyz/"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="btn btn--accent"
                    >
                        "在 Bilibili 打开"
                    </a>
                </div>

                <div class="trailer-hero__stage">
                    <a
                        class="screening__frame"
                        href="https://www.bilibili.com/video/BV1U6qDBmEyz/"
                        target="_blank"
                        rel="noopener noreferrer"
                        aria-label="在 Bilibili 观看年度预告"
                    >
                        <span class="screening__play" aria-hidden="true">"▶"</span>
                        <span class="screening__label mono">"PLAY ON BILIBILI"</span>
                        <span class="screening__time mono">"01:42"</span>
                    </a>

                    <aside class="trailer-hero__rail">
                        <span class="eyebrow">"Specification"</span>
                        <dl class="meta-list">
                            {META
                                .into_iter()
                                .map(|(key, value)| {
                                    view! {
                                        <div class="meta-list__row">
                                            <dt class="mono">{key}</dt>
                                            <dd>{value}</dd>
                                        </div>
                                    }
                                })
                                .collect_view()}
                        </dl>
                    </aside>
                </div>

                <div class="trailer-hero__timeline">
                    {CHAPTERS
                        .iter()
                        .map(|chapter| {
                            let ratio = chapter.at as f32 / RUNTIME_SECS as f32 * 100.0;
                            view! {
                                <div class="trailer-hero__tick" style=format!("left: {:.2}%", ratio)>
                                    <span class="trailer-hero__tick-time mono">{chapter.code}</span>
                                    <span class="trailer-hero__tick-title">{chapter.title}</span>
                                </div>
                            }
                        })
                        .collect_view()}
                    <span class="trailer-hero__end mono">"END // 01:42"</span>
                </div>
            </div>
        </section>

        <section class="section section--ink">
            <div class="shell">
                <SectionHead
                    index="03.1"
                    eyebrow="Chapters"
                    title="时间轴"
                />
                <div class="chapter-list">
                    {CHAPTERS
                        .iter()
                        .map(|chapter| {
                            view! {
                                <div class="chapter">
                                    <span class="chapter__time mono">{chapter.code}</span>
                                    <h3 class="chapter__title">{chapter.title}</h3>
                                    <p class="chapter__body">{chapter.body}</p>
                                </div>
                            }
                        })
                        .collect_view()}
                </div>
            </div>
        </section>
    }
}
