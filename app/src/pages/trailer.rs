use leptos::prelude::*;
use leptos_meta::{Meta, Title};

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

    const CHAPTERS: [(&str, &str, &str); 4] = [
        ("00:00", "开场", "工作室标识与年度基调"),
        ("00:18", "世界", "四个项目的片段交替"),
        ("00:54", "系统", "引擎与工具链的快速剪辑"),
        ("01:24", "署名", "团队与致谢"),
    ];

    view! {
        <Title text="Trailer — ASDF Studio"/>
        <Meta
            name="description"
            content="ASDF Studio 2026 年度预告片，于 Bilibili 观看。"
        />
        <section class="page-hero page-hero--compact">
            <div class="shell page-hero__grid">
                <span class="index-mark">"03"</span>
                <div class="page-hero__text">
                    <span class="eyebrow">"Screening Room"</span>
                    <h1 class="display">"年度预告"</h1>
                    <p class="lede">
                        "一次对四个世界的并行剪辑。没有旁白，只有系统运转的声音。"
                    </p>
                </div>
            </div>
        </section>

        <section class="section">
            <div class="shell screening">
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

                <aside class="screening__meta">
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
                    <a
                        href="https://www.bilibili.com/video/BV1U6qDBmEyz/"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="btn btn--accent btn--block"
                    >
                        "在 Bilibili 打开"
                    </a>
                </aside>
            </div>
        </section>

        <section class="section section--ink">
            <div class="shell">
                <header class="section-head">
                    <span class="section-head__index mono">"03.1"</span>
                    <div class="section-head__text">
                        <span class="eyebrow">"Chapters"</span>
                        <h2 class="section-head__title">"时间轴"</h2>
                    </div>
                </header>
                <div class="chapter-list">
                    {CHAPTERS
                        .into_iter()
                        .map(|(time, title, body)| {
                            view! {
                                <div class="chapter">
                                    <span class="chapter__time mono">{time}</span>
                                    <h3 class="chapter__title">{title}</h3>
                                    <p class="chapter__body">{body}</p>
                                </div>
                            }
                        })
                        .collect_view()}
                </div>
            </div>
        </section>
    }
}
