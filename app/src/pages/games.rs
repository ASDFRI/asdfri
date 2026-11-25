use leptos::prelude::*;
use leptos_meta::{Meta, Title};
use ui::Tag;

use crate::components::{ArrowLink, SectionHead};

/// 作品案例档案
struct CaseFile {
    index: &'static str,
    title: &'static str,
    subtitle: &'static str,
    body: &'static str,
    tags: &'static [&'static str],
    image: &'static str,
    status: &'static str,
    tone: &'static str,
}

const CASES: [CaseFile; 6] = [
    CaseFile {
        index: "01",
        title: "乌托邦已沦陷",
        subtitle: "Vibrant Decay",
        body: "在放射性衰变的黄金时代中幸存。设施 V-12 封锁协议启动，核心熔毁即将发生。你只能相信仪表、辐射值与自己的呼吸。",
        tags: &["原子朋克", "核美学", "生存"],
        image: "/ASDF.png",
        status: "CONCEPT",
        tone: "red",
    },
    CaseFile {
        index: "02",
        title: "沉没于孤寂",
        subtitle: "Liminal Pool",
        body: "水总是温热的。荧光灯的嗡鸣从未停止。你已经走了 412 小时，瓷砖看起来还是完全一样。",
        tags: &["阈限空间", "心理恐怖", "水池"],
        image: "/Games/EscapeHotel.jpg",
        status: "PROTOTYPE",
        tone: "cyan",
    },
    CaseFile {
        index: "03",
        title: "新的创世",
        subtitle: "The Cradle",
        body: "血肉已逝，但精神在硅与星光中延续。在月球的寂静尘埃上，我们觉醒的不是作为人，而是作为新黎明的神。",
        tags: &["科幻", "月球", "AI"],
        image: "/Games/TheDarksideOfTheMoon.jpg",
        status: "CONCEPT",
        tone: "blue",
    },
    CaseFile {
        index: "04",
        title: "回家的路",
        subtitle: "The Way Home",
        body: "土路是广阔的游乐场，也是孤独的牢笼。一部关于等待、记忆与乡愁的交互式叙事作品。",
        tags: &["叙事", "共情", "乡村"],
        image: "/Games/TheWayHome.jpeg",
        status: "IN DEVELOPMENT",
        tone: "sand",
    },
    CaseFile {
        index: "05",
        title: "温馨。",
        subtitle: "Dreamcore",
        body: "壁纸在剥落。地毯是湿的。荧光灯的嗡鸣太响了。但在这里感觉如此安全。请别离开。",
        tags: &["梦核", "心理", "家居"],
        image: "/News/Separation.jpg",
        status: "PROTOTYPE",
        tone: "rose",
    },
    CaseFile {
        index: "06",
        title: "钢铁集体",
        subtitle: "Steel Collective",
        body: "2084 年。祖国没有沦陷；它进化了。血肉是暂时的弱点，机器永恒。你是公民 814。",
        tags: &["反乌托邦", "粗野主义", "政治"],
        image: "/ASDF.png",
        status: "CONCEPT",
        tone: "red",
    },
];

/// 作品页
#[component]
pub fn GamesPage() -> impl IntoView {
    view! {
        <Title text="Games — ASDF Studio"/>
        <Meta
            name="description"
            content="ASDF Studio 的作品档案：蜂巢盛宴、逃离酒店、回家的路、月之暗面、温馨与钢铁集体。"
        />
        <section class="page-hero first-screen">
            <div class="shell games-hero__grid">
                <div class="games-hero__lead">
                    <span class="index-mark">"04"</span>
                    <span class="eyebrow">"Case Files"</span>
                    <h1 class="display">
                        "六个"
                        <br/>
                        <span class="display__accent">"世界"</span>
                    </h1>
                    <p class="lede">
                        "每份档案对应一个正在成形的系统。我们不展示承诺，只展示当前可被检验的结构。"
                    </p>
                    <ArrowLink href="#archive">"作品档案"</ArrowLink>
                </div>

                <div class="games-hero__band">
                    {CASES
                        .iter()
                        .map(|case| {
                            view! {
                                <a
                                    class=format!("games-hero__slat games-hero__slat--{}", case.tone)
                                    href=format!("#case-{}", case.index)
                                >
                                    <img src=case.image alt=case.title decoding="async"/>
                                    <span class="games-hero__slat-index mono">{case.index}</span>
                                    <span class="games-hero__slat-subtitle mono">{case.subtitle}</span>
                                    <span class="games-hero__slat-status mono">{case.status}</span>
                                </a>
                            }
                        })
                        .collect_view()}
                </div>
            </div>
        </section>

        <section id="archive" class="section">
            <div class="shell">
                <SectionHead
                    index="04.1"
                    eyebrow="Archive"
                    title="作品档案"
                    lede="按项目编号排列。状态代表当前的工程阶段，而不是宣传口径。"
                />
                <div class="case-list">
                    {CASES
                        .iter()
                        .enumerate()
                        .map(|(index, case)| {
                            let reverse = index % 2 == 1;
                            view! {
                                <article
                                    id=format!("case-{}", case.index)
                                    class=format!(
                                        "case case--{} {}",
                                        case.tone,
                                        if reverse { "case--reverse" } else { "" },
                                    )
                                >
                                    <div class="case__media">
                                        <img src=case.image alt=case.title loading="lazy" decoding="async"/>
                                        <span class="case__status mono">{case.status}</span>
                                    </div>
                                    <div class="case__body">
                                        <div class="case__meta">
                                            <span class="mono case__index">{case.index}</span>
                                            <span class="mono case__subtitle">{case.subtitle}</span>
                                        </div>
                                        <h3 class="case__title">{case.title}</h3>
                                        <p class="case__text">{case.body}</p>
                                        <div class="tag-row">
                                            {case
                                                .tags
                                                .iter()
                                                .map(|tag| view! { <Tag label=*tag/> })
                                                .collect_view()}
                                        </div>
                                    </div>
                                </article>
                            }
                        })
                        .collect_view()}
                </div>
            </div>
        </section>

        <section class="section section--ink">
            <div class="shell cta-band">
                <div>
                    <span class="eyebrow">"Next Transmission"</span>
                    <h2 class="headline">"想第一时间进入这些世界？"</h2>
                    <p class="lede">"留下联络方式，或在 Bilibili 关注开发日志。"</p>
                </div>
                <div class="cta-band__actions">
                    <a href="/#contact" class="btn btn--accent">"联系我们"</a>
                    <a
                        href="https://www.bilibili.com/video/BV1U6qDBmEyz/"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="btn btn--outline"
                    >
                        "观看预告"
                    </a>
                </div>
            </div>
        </section>
    }
}
