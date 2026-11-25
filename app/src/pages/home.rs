use api::{
    game::{Game, get_games},
    member::{Member, get_members},
    site::{HeroContent, get_hero},
};
use leptos::prelude::*;
use leptos_meta::{Meta, Title};
use ui::{Panel, Skeleton, Tag};

use crate::{
    components::{ArrowLink, SectionHead},
    site::SiteVariantSignal,
};

/// 首页
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Title text="ASDF Studio"/>
        <Meta
            name="description"
            content="ASDF Studio 是一个由工程师与艺术家组成的独立工作室，以严格的结构与克制的表达构建虚拟世界。"
        />
        <HomeHero/>
        <StudioSection/>
        <NewsSection/>
        <GamesSection/>
        <TeamSection/>
        <CareersSection/>
        <ContactSection/>
    }
}

/// 首屏
#[component]
fn HomeHero() -> impl IntoView {
    let variant = SiteVariantSignal::use_state();
    let hero = Resource::new(move || variant.get(), get_hero);

    view! {
        <Suspense fallback=move || {
            view! { <HeroMarkup content=HeroContent::defaults(variant.get())/> }
        }>
            {move || {
                hero.get()
                    .map(|result| match result {
                        Ok(content) => view! { <HeroMarkup content=content/> }.into_any(),
                        Err(_) => {
                            view! { <HeroMarkup content=HeroContent::defaults(variant.get())/> }
                                .into_any()
                        }
                    })
            }}
        </Suspense>
    }
}

/// 首屏内容
#[component]
fn HeroMarkup(content: HeroContent) -> impl IntoView {
    let HeroContent {
        eyebrow,
        location,
        title,
        lede,
        primary_label,
        primary_href,
        secondary_label,
        secondary_href,
        media_tag,
        media_poster,
        media_video,
        strip,
    } = content;
    let has_media = !media_video.is_empty();
    let has_strip = !strip.is_empty();
    let grid_class = match (has_media, has_strip) {
        (true, true) => "shell home-hero__grid",
        (true, false) => "shell home-hero__grid home-hero__grid--no-strip",
        (false, true) => "shell home-hero__grid home-hero__grid--no-media",
        (false, false) => {
            "shell home-hero__grid home-hero__grid--no-media home-hero__grid--no-strip"
        }
    };

    let location_view = (!location.is_empty())
        .then(|| view! { <span class="eyebrow">{location}</span> }.into_any());
    let title_view = title
        .into_iter()
        .enumerate()
        .map(|(index, line)| match index {
            0 => view! { {line} }.into_any(),
            1 => view! { <br/><span class="display__accent">{line}</span> }.into_any(),
            _ => view! { <br/>{line} }.into_any(),
        })
        .collect_view();
    let lede_view = (!lede.is_empty()).then(|| view! { <p class="lede">{lede}</p> }.into_any());

    let primary_view = (!primary_label.is_empty() && !primary_href.is_empty()).then(|| {
        view! { <a href=primary_href class="btn btn--solid">{primary_label}</a> }.into_any()
    });
    let secondary_view = (!secondary_label.is_empty() && !secondary_href.is_empty()).then(|| {
        view! { <a href=secondary_href class="btn btn--outline">{secondary_label}</a> }.into_any()
    });
    let actions_view = (primary_view.is_some() || secondary_view.is_some()).then(|| {
        view! {
            <div class="home-hero__actions">
                {primary_view}
                {secondary_view}
            </div>
        }
        .into_any()
    });

    let media_view = has_media.then(|| {
        let poster = media_poster;
        let tag = (!media_tag.is_empty())
            .then(|| view! { <span class="home-hero__media-tag mono">{media_tag}</span> });
        view! {
            <div class="home-hero__media">
                <video
                    autoplay=true
                    loop=true
                    muted=true
                    playsinline=true
                    poster=poster
                    aria-hidden="true"
                >
                    <source src=media_video type="video/mp4"/>
                </video>
                {tag}
            </div>
        }
        .into_any()
    });

    let strip_view = has_strip.then(|| {
        view! {
            <div class="home-hero__strip" aria-hidden="true">
                {strip.into_iter().map(|label| view! { <span>{label}</span> }).collect_view()}
            </div>
        }
        .into_any()
    });

    view! {
        <section class="home-hero first-screen">
            <div class=grid_class>
                <div class="home-hero__meta">
                    <span class="eyebrow">{eyebrow}</span>
                    {location_view}
                </div>

                <div class="home-hero__title">
                    <span class="index-mark">"01"</span>
                    <h1 class="display">{title_view}</h1>
                    {lede_view}
                    {actions_view}
                </div>

                {media_view}

                {strip_view}
            </div>
        </section>
    }
}

/// 工作室介绍
#[component]
fn StudioSection() -> impl IntoView {
    view! {
        <section id="studio" class="section section--ink">
            <div class="shell intro-grid">
                <SectionHead
                    index="02"
                    eyebrow="The Studio"
                    title="结构先行"
                    lede="我们相信形式不是装饰，而是规则的外化。每一次排版、每一行代码、每一个交互反馈，都必须能够被解释。"
                />
                <div class="intro-grid__body">
                    <p class="statement">
                        "绝不妥协，只为纯粹的游戏玩法。我们把复杂系统拆解成清晰的层级，再以最少的元素重新组合。"
                    </p>
                    <div class="stat-grid">
                        <div class="stat">
                            <span class="stat__value">"UE5"</span>
                            <span class="stat__label">"主引擎"</span>
                        </div>
                        <div class="stat">
                            <span class="stat__value">"RUST"</span>
                            <span class="stat__label">"服务端"</span>
                        </div>
                        <div class="stat">
                            <span class="stat__value">"100%"</span>
                            <span class="stat__label">"自研管线"</span>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}

/// 新闻动态
#[component]
fn NewsSection() -> impl IntoView {
    const NEWS: [(&str, &str, &str, &str, &str); 3] = [
        (
            "2026.03.15",
            "开发日志",
            "全新官网上线",
            "没有大张旗鼓，只想换个舒服的方式和你见面。少一点刻板，多一些呼吸感——关于我们想做的事、在做的事，现在都能在这里找到更清晰的注脚。",
            "/ASDF.png",
        ),
        (
            "2026.01.10",
            "公告",
            "短暂的分离",
            "我们没有走远，只是暂时散落在不同的经纬度里，过着各自的日常。偶尔视频，看见彼此身后的风景变了，但说话的语气还是老样子。",
            "/News/Separation.jpg",
        ),
        (
            "2025.11.25",
            "里程碑",
            "梦开始的地方",
            "从一张草图到一个可运行的世界，我们把最初的想法保留至今，并让它成为所有系统的基准。",
            "/Games/TheDarksideOfTheMoon.jpg",
        ),
    ];

    view! {
        <section id="news" class="section">
            <div class="shell">
                <SectionHead
                    index="03"
                    eyebrow="Transmissions"
                    title="最新动态"
                    lede="工作室公告、开发日志与阶段性记录。所有信息按时间排列，不隐藏过程。"
                />
                <div class="news-grid">
                    {NEWS
                        .into_iter()
                        .map(|(date, kind, title, body, image)| {
                            view! {
                                <article class="news-card">
                                    <div class="news-card__media">
                                        <img src=image alt=title loading="lazy"/>
                                        <span class="news-card__kind mono">{kind}</span>
                                    </div>
                                    <span class="mono news-card__date">{date}</span>
                                    <h3 class="news-card__title">{title}</h3>
                                    <p class="news-card__body">{body}</p>
                                </article>
                            }
                        })
                        .collect_view()}
                </div>
            </div>
        </section>
    }
}

/// 游戏作品
#[component]
fn GamesSection() -> impl IntoView {
    let games = Resource::new(|| (), |_| get_games());

    view! {
        <section id="games" class="section section--ink">
            <div class="shell">
                <SectionHead
                    index="04"
                    eyebrow="Our Worlds"
                    title="精选游戏"
                    lede="四个正在成形中的世界。每一个都以不同的规则回答同一个问题：人如何与系统相处。"
                />
                <div class="dossier-grid">
                    <Suspense fallback=move || {
                        view! {
                            <Skeleton class="dossier-skeleton"/>
                            <Skeleton class="dossier-skeleton"/>
                            <Skeleton class="dossier-skeleton"/>
                        }
                    }>
                        {move || {
                            games
                                .get()
                                .map(|result| match result {
                                    Ok(games) if games.is_empty() => {
                                        view! {
                                            <p class="notice col-span-full">
                                                "全新世界正在孕育中，暂无公开的作品记录。"
                                            </p>
                                        }
                                            .into_any()
                                    }
                                    Ok(games) => {
                                        view! {
                                            <For
                                                each=move || games.clone().into_iter().enumerate()
                                                key=|(_, game)| game.id
                                                children=move |(index, game)| {
                                                    view! { <GameDossier game=game index=index/> }
                                                }
                                            />
                                        }
                                            .into_any()
                                    }
                                    Err(error) => {
                                        view! {
                                            <p class="notice notice--error col-span-full">
                                                {format!("作品档案读取失败：{error}")}
                                            </p>
                                        }
                                            .into_any()
                                    }
                                })
                        }}
                    </Suspense>
                </div>
                <div class="section-foot">
                    <ArrowLink href="/games">"查看全部作品档案"</ArrowLink>
                </div>
            </div>
        </section>
    }
}

/// 单个作品档案
#[component]
fn GameDossier(game: Game, index: usize) -> impl IntoView {
    let Game {
        id: _,
        name,
        screenshots,
        description,
        tags,
        link,
        sort_order: _,
    } = game;
    let cover = screenshots.first().cloned().unwrap_or_else(|| "/ASDF.png".to_string());
    let cover_signal = RwSignal::new(cover);
    let alt = name.clone();

    view! {
        <article class="dossier">
            <span class="dossier__index mono">{format!("{:02}", index + 1)}</span>
            <div class="dossier__media">
                <img
                    src=move || cover_signal.get()
                    on:error=move |_| cover_signal.set("/ASDF.png".to_string())
                    alt=alt
                    loading="lazy"
                    decoding="async"
                />
            </div>
            <div class="dossier__body">
                <h3 class="dossier__title">{name}</h3>
                {description.map(|text| view! { <p class="dossier__text">{text}</p> })}
                <div class="tag-row">
                    {tags.into_iter().map(|tag| view! { <Tag label=tag/> }).collect_view()}
                </div>
                {link.map(|href| view! { <ArrowLink href=href>"进入世界"</ArrowLink> })}
            </div>
        </article>
    }
}

/// 团队
#[component]
fn TeamSection() -> impl IntoView {
    let members = Resource::new(|| (), |_| get_members());

    view! {
        <section id="team" class="section">
            <div class="shell">
                <SectionHead
                    index="05"
                    eyebrow="The Architects"
                    title="认识团队成员"
                    lede="五个岗位，一条明确的职责链。每个人对自己的模块负责，并对整体结构保持敏感。"
                />
                <div class="people-grid">
                    <Suspense fallback=move || {
                        view! {
                            <Skeleton class="person-skeleton"/>
                            <Skeleton class="person-skeleton"/>
                            <Skeleton class="person-skeleton"/>
                            <Skeleton class="person-skeleton"/>
                        }
                    }>
                        {move || {
                            members
                                .get()
                                .map(|result| match result {
                                    Ok(members) if members.is_empty() => {
                                        view! {
                                            <p class="notice col-span-full">"暂无公开的成员档案。"</p>
                                        }
                                            .into_any()
                                    }
                                    Ok(members) => {
                                        view! {
                                            <For
                                                each=move || members.clone()
                                                key=|member| member.id
                                                children=move |member| view! { <PersonCard member=member/> }
                                            />
                                        }
                                            .into_any()
                                    }
                                    Err(error) => {
                                        view! {
                                            <p class="notice notice--error col-span-full">
                                                {format!("成员档案读取失败：{error}")}
                                            </p>
                                        }
                                            .into_any()
                                    }
                                })
                        }}
                    </Suspense>
                </div>
            </div>
        </section>
    }
}

/// 单个成员卡片
#[component]
fn PersonCard(member: Member) -> impl IntoView {
    let Member {
        id: _,
        name,
        icon,
        description: _,
        slogan,
        tags,
        position,
        sort_order: _,
    } = member;
    let avatar = icon.unwrap_or_else(|| "/ASDF.png".to_string());
    let avatar_signal = RwSignal::new(avatar);
    let alt = name.clone();

    view! {
        <article class="person">
            <div class="person__media">
                <img
                    src=move || avatar_signal.get()
                    on:error=move |_| avatar_signal.set("/ASDF.png".to_string())
                    alt=alt
                    loading="lazy"
                    decoding="async"
                />
            </div>
            <div class="person__body">
                <div class="person__head">
                    <h3 class="person__name">{name}</h3>
                    {position.map(|value| view! { <span class="mono person__role">{value}</span> })}
                </div>
                {slogan.map(|value| view! { <p class="person__slogan">{value}</p> })}
                <div class="tag-row">
                    {tags.into_iter().map(|tag| view! { <Tag label=tag/> }).collect_view()}
                </div>
            </div>
        </article>
    }
}

/// 招聘
#[component]
fn CareersSection() -> impl IntoView {
    const JOBS: [(&str, &str, &str); 3] = [
        ("高级战斗设计师", "游戏设计", "远程 / 武汉"),
        ("概念设计艺术家", "概念艺术", "远程 / 武汉"),
        ("高级技术艺术家", "艺术 & 技术", "远程 / 武汉"),
    ];

    view! {
        <section id="careers" class="section section--ink">
            <div class="shell split-grid">
                <div class="split-grid__aside">
                    <SectionHead
                        index="06"
                        eyebrow="Forge the Future"
                        title="加入我们"
                        lede="我们始终寻觅富有远见的人才。若你勇于探索未知，从不满足于「尚可」，你的下一段征程便从此开启。"
                    />
                    <a href="#contact" class="btn btn--accent">"查看所有招聘岗位"</a>
                </div>
                <div class="job-list">
                    {JOBS
                        .into_iter()
                        .map(|(title, department, location)| {
                            view! {
                                <a href="#contact" class="job">
                                    <div class="job__main">
                                        <h3 class="job__title">{title}</h3>
                                        <div class="job__meta mono">
                                            <span>{department}</span>
                                            <span>"·"</span>
                                            <span>"全职"</span>
                                        </div>
                                    </div>
                                    <div class="job__side">
                                        <span class="mono">{location}</span>
                                        <span class="job__arrow" aria-hidden="true">"→"</span>
                                    </div>
                                </a>
                            }
                        })
                        .collect_view()}
                </div>
            </div>
        </section>
    }
}

/// 联系
#[component]
fn ContactSection() -> impl IntoView {
    const CHANNELS: [(&str, &str); 3] =
        [("01", "合作 & 出版"), ("02", "新闻 & 媒体资料包"), ("03", "一般咨询")];

    view! {
        <section id="contact" class="section">
            <div class="shell split-grid">
                <SectionHead
                    index="07"
                    eyebrow="Initiate Contact"
                    title="我们一起打造一部史诗"
                    lede="无论您是寻找下一部作品的出版商、寻求新闻资料包的记者，还是希望合作的远见者，我们的沟通渠道都畅通无阻。"
                />
                <div class="contact-list">
                    {CHANNELS
                        .into_iter()
                        .map(|(index, label)| {
                            view! {
                                <div class="contact-item">
                                    <span class="contact-item__index mono">{index}</span>
                                    <span class="contact-item__label">{label}</span>
                                    <a href="mailto:asdf@gmail.com" class="contact-item__mail">
                                        "asdf@gmail.com"
                                    </a>
                                </div>
                            }
                        })
                        .collect_view()}
                    <Panel dark=true class="contact-panel">
                        <span class="eyebrow">"Studio Position"</span>
                        <p class="contact-panel__text">
                            "我们只接受能够被清晰定义的合作。请在邮件中说明目标、范围与时间。"
                        </p>
                    </Panel>
                </div>
            </div>
        </section>
    }
}
