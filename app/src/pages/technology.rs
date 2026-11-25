use leptos::prelude::*;
use leptos_meta::{Meta, Title};
use ui::Tag;

use crate::components::{ArrowLink, SectionHead};

/// 引擎规格
struct EngineSpec {
    mark: &'static str,
    label: &'static str,
    title: &'static str,
    body: &'static str,
    tags: &'static [&'static str],
    code: &'static [&'static str],
    tone: &'static str,
}

/// 工具规格
struct ToolSpec {
    mark: &'static str,
    name: &'static str,
    category: &'static str,
    body: &'static str,
    tone: &'static str,
}

/// 基础设施卡片
struct InfraSpec {
    title: &'static str,
    metric: &'static str,
    body: &'static str,
    tags: &'static [&'static str],
    wide: bool,
}

/// 美学切片
struct AestheticSpec {
    index: &'static str,
    title: &'static str,
    body: &'static str,
    tags: &'static [&'static str],
    tone: &'static str,
}

const ENGINES: [EngineSpec; 3] = [
    EngineSpec {
        mark: "U",
        label: "Heavy Firepower",
        title: "虚幻引擎 5",
        body: "我们的重火力。利用 Nanite 与 Lumen 实现不妥协的写实渲染，同时将逻辑构建于久经考验的 C++ 基础之上。",
        tags: &["C++ 与蓝图", "Nanite / Lumen", "GAS 架构"],
        code: &[
            "UCLASS(Blueprintable)",
            "class USingularisCombatComponent : public UActorComponent {",
            "    GENERATED_BODY()",
            "};",
        ],
        tone: "blue",
    },
    EngineSpec {
        mark: "B",
        label: "Precision Instrument",
        title: "Bevy (Rust)",
        body: "精准的手术刀。当需要模拟数万个实体而不掉帧时，我们转向 Rust 与 Bevy。严格的内存安全与纯数据驱动的 ECS 架构。",
        tags: &["Rust", "纯 ECS", "数据驱动"],
        code: &[
            "fn player_movement_system(",
            "    mut query: Query<(&mut Transform, &Player)>,",
            "    time: Res<Time>,",
            ") { /* 无锁并行 */ }",
        ],
        tone: "amber",
    },
    EngineSpec {
        mark: "G",
        label: "Rapid Prototyping",
        title: "Godot 引擎",
        body: "万能工具。对于快速原型、风格化 3D 与复杂 2D 机制，Godot 提供了无与伦比的节点式流程。开源自由，零授权摩擦。",
        tags: &["GDScript 与 C#", "节点场景树", "快速原型"],
        code: &[
            "# Player.gd",
            "extends CharacterBody3D",
            "func _physics_process(delta):",
            "    move_and_slide()",
        ],
        tone: "cyan",
    },
];

const TOOLS: [ToolSpec; 4] = [
    ToolSpec {
        mark: "Bl",
        name: "Blender",
        category: "3D / 动画",
        body: "多边形建模、UV 展开与绑定的主要套件。通过内部 Python 脚本扩展，实现一键导出到引擎。",
        tone: "orange",
    },
    ToolSpec {
        mark: "Pt",
        name: "Substance",
        category: "纹理",
        body: "参数化材质生成与 PBR 纹理。确保每个表面在 Lumen 光照下都能正确物理反应。",
        tone: "green",
    },
    ToolSpec {
        mark: "Hd",
        name: "Houdini",
        category: "程序化",
        body: "基于节点的程序化生成，适用于地形、城市与破坏模拟。VEX 脚本提供无限扩展性。",
        tone: "yellow",
    },
    ToolSpec {
        mark: "Zb",
        name: "ZBrush",
        category: "雕刻",
        body: "高多边形角色与生物雕刻。捕捉微观表面细节，随后烘焙为高性能的游戏就绪网格。",
        tone: "red",
    },
];

const INFRA: [InfraSpec; 4] = [
    InfraSpec {
        title: "高并发微服务",
        metric: "99.99%",
        body: "基于 Rust 与 Axum 构建极速、内存安全的 API 网关、玩家遥测端点与持久化状态服务。零垃圾回收停顿，纯裸机性能。",
        tags: &["Axum", "Tokio", "SQLx"],
        wide: true,
    },
    InfraSpec {
        title: "响应式 Web",
        metric: "SSR",
        body: "从官网到内部管理面板，使用 Leptos 与 WebAssembly 交付高度响应式的前端，并保持首屏可索引。",
        tags: &["Leptos", "Wasm", "SSR"],
        wide: false,
    },
    InfraSpec {
        title: "专用服务器集群",
        metric: "< 50ms",
        body: "无缝部署虚幻引擎 Linux 专用服务器。容器化与编排让服务器容量随并发自动伸缩。",
        tags: &["Docker", "K8s", "Fleet"],
        wide: false,
    },
    InfraSpec {
        title: "流水线与身份",
        metric: "CI/CD",
        body: "零人工构建。流水线自动编译、打包并分发版本；玩家身份、云存档与成就通过统一身份层安全管理。",
        tags: &["CI/CD", "EOS", "Observability"],
        wide: true,
    },
];

const AESTHETICS: [AestheticSpec; 4] = [
    AestheticSpec {
        index: "01",
        title: "原子朋克与模拟",
        body: "灵感来自废土的锈蚀壮丽与粗野主义乌托邦之梦。我们热爱模拟刻度盘、真空管以及走向灾难的乐观复古未来。",
        tags: &["苏式粗野主义", "复古未来"],
        tone: "amber",
    },
    AestheticSpec {
        index: "02",
        title: "阈限空间",
        body: "未曾涉足之地的诡异熟悉感。我们将梦核美学融入环境，唤起深层怀旧与深刻孤寂的交织。",
        tags: &["梦核", "阈限"],
        tone: "cyan",
    },
    AestheticSpec {
        index: "03",
        title: "优雅怪诞",
        body: "在恐怖中探寻精致。生物与角色设计融合高级时装的优雅与生物机械的身体恐怖。",
        tags: &["生物机械", "怪诞"],
        tone: "red",
    },
    AestheticSpec {
        index: "04",
        title: "美学即玩法",
        body: "美术指导不是外衣，而是基础机制。走廊的不安布局引导紧张感；机械的咔哒声决定战斗节奏。我们不只设计关卡，我们工程化心理状态。",
        tags: &["叙事", "心理"],
        tone: "ink",
    },
];

/// 技术页
#[component]
pub fn TechnologyPage() -> impl IntoView {
    view! {
        <Title text="Technology — ASDF Studio"/>
        <Meta
            name="description"
            content="ASDF Studio 的技术栈：虚幻引擎 5、Bevy、Godot、程序化资产管线与 Rust/Axum 后端。"
        />
        <TechnologyHero/>
        <EnginesSection/>
        <PipelineSection/>
        <InnovationSection/>
        <InfrastructureSection/>
        <AestheticsSection/>
    }
}

/// 技术栈索引行
struct StackEntry {
    index: &'static str,
    name: &'static str,
    category: &'static str,
}

/// 技术栈索引：按职能登记的工作室全套工具
const STACK: [StackEntry; 8] = [
    StackEntry {
        index: "01",
        name: "Unreal Engine 5",
        category: "RENDERING",
    },
    StackEntry {
        index: "02",
        name: "Godot",
        category: "OPEN SOURCE",
    },
    StackEntry {
        index: "03",
        name: "Bevy",
        category: "ECS / RUST",
    },
    StackEntry {
        index: "04",
        name: "Blender",
        category: "PROCEDURAL",
    },
    StackEntry {
        index: "05",
        name: "C++ & Rust",
        category: "SYSTEMS",
    },
    StackEntry {
        index: "06",
        name: "Leptos",
        category: "ISOMORPHIC UI",
    },
    StackEntry {
        index: "07",
        name: "Axum",
        category: "HTTP SERVICE",
    },
    StackEntry {
        index: "08",
        name: "SQLite",
        category: "STORAGE",
    },
];

/// 技术首屏：左叙事列配右侧全高技术栈账本
#[component]
fn TechnologyHero() -> impl IntoView {
    view! {
        <section class="page-hero first-screen">
            <div class="shell tech-hero__grid">
                <div class="tech-hero__lead">
                    <span class="index-mark">"02"</span>
                    <span class="eyebrow">"The Forge"</span>
                    <h1 class="display">
                        "引擎"
                        <br/>
                        <span class="display__accent">"创造"</span>
                    </h1>
                    <p class="lede">
                        "我们不被单一工具束缚。从照片级 AAA 世界到数据驱动的 ECS 模拟，我们驾驭合适的技术，锻造无与伦比的交互体验。"
                    </p>
                    <ArrowLink href="#core-engines">"引擎档案"</ArrowLink>
                </div>

                <aside class="tech-hero__ledger">
                    <div class="tech-hero__ledger-head mono">
                        <span>"STACK INDEX"</span>
                        <span>{format!("{:02} ENTRIES", STACK.len())}</span>
                    </div>
                    {STACK
                        .iter()
                        .map(|entry| {
                            view! {
                                <div class="tech-hero__row">
                                    <span class="tech-hero__row-index mono">{entry.index}</span>
                                    <span class="tech-hero__row-name">{entry.name}</span>
                                    <span class="tech-hero__row-category mono">{entry.category}</span>
                                </div>
                            }
                        })
                        .collect_view()}
                </aside>
            </div>
        </section>
    }
}

/// 核心引擎
#[component]
fn EnginesSection() -> impl IntoView {
    view! {
        <section id="core-engines" class="section section--ink">
            <div class="shell">
                <SectionHead
                    index="02.1"
                    eyebrow="Core Architectures"
                    title="选择你的武器"
                    lede="每种引擎只解决一类问题。我们按问题选择工具，而不是按偏好。"
                />
                <div class="engine-list">
                    {ENGINES
                        .iter()
                        .enumerate()
                        .map(|(index, engine)| {
                            let reverse = index % 2 == 1;
                            view! {
                                <article class=format!(
                                    "engine {}",
                                    if reverse { "engine--reverse" } else { "" },
                                )>
                                    <div class="engine__body">
                                        <span class="engine__mark mono">{engine.mark}</span>
                                        <span class="eyebrow">{engine.label}</span>
                                        <h3 class="engine__title">{engine.title}</h3>
                                        <p class="engine__text">{engine.body}</p>
                                        <div class="tag-row">
                                            {engine
                                                .tags
                                                .iter()
                                                .map(|tag| view! { <Tag label=*tag/> })
                                                .collect_view()}
                                        </div>
                                    </div>
                                    <div class=format!("engine__media engine__media--{}", engine.tone)>
                                        <div class="code-panel">
                                            {engine
                                                .code
                                                .iter()
                                                .map(|line| view! { <span class="code-panel__line">{*line}</span> })
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
    }
}

/// 资产管线
#[component]
fn PipelineSection() -> impl IntoView {
    view! {
        <section id="art-pipeline" class="section">
            <div class="shell">
                <SectionHead
                    index="02.2"
                    eyebrow="The Digital Forge"
                    title="资产管线"
                    lede="代码赋予生命，艺术赋予灵魂。完全程序化且非破坏性的资产管线确保每个多边形与像素都为次世代硬件而优化。"
                />
                <div class="tool-grid">
                    {TOOLS
                        .iter()
                        .map(|tool| {
                            view! {
                                <article class=format!("tool-card tool-card--{}", tool.tone)>
                                    <div class="tool-card__head">
                                        <span class="tool-card__mark mono">{tool.mark}</span>
                                        <span class="mono tool-card__category">{tool.category}</span>
                                    </div>
                                    <h3 class="tool-card__title">{tool.name}</h3>
                                    <p class="tool-card__text">{tool.body}</p>
                                </article>
                            }
                        })
                        .collect_view()}
                </div>
            </div>
        </section>
    }
}

/// 创意与概念
#[component]
fn InnovationSection() -> impl IntoView {
    view! {
        <section id="innovation" class="section section--ink">
            <div class="shell innovation">
                <div class="innovation__text">
                    <SectionHead
                        index="02.3"
                        eyebrow="Creative Genesis"
                        title="无界概念"
                        lede="技术不仅用于执行，更是我们的共创者。我们将工程逻辑编织进富有远见的概念艺术，打破抽象思维与交互现实之间的速度屏障。"
                    />
                    <div class="feature-list">
                        <div class="feature">
                            <h3 class="feature__title">"AI 辅助构思"</h3>
                            <p class="feature__text">
                                "利用生成式模型探索情绪板、实时风格迁移与角色变体，但最终决策始终由人完成。"
                            </p>
                        </div>
                        <div class="feature">
                            <h3 class="feature__title">"程序化世界构建"</h3>
                            <p class="feature__text">
                                "将概念艺术直接转化为 Houdini 几何节点，实现即时、可扩展且可复现的世界生成。"
                            </p>
                        </div>
                    </div>
                </div>
                <div class="innovation__visual">
                    <div class="blueprint">
                        <span class="blueprint__tag mono">"CONCEPT // V-08"</span>
                        <div class="blueprint__grid"></div>
                        <div class="blueprint__axis blueprint__axis--x"></div>
                        <div class="blueprint__axis blueprint__axis--y"></div>
                        <span class="blueprint__label">"逻辑融合矩阵"</span>
                    </div>
                </div>
            </div>
        </section>
    }
}

/// 基础设施
#[component]
fn InfrastructureSection() -> impl IntoView {
    view! {
        <section id="infrastructure" class="section">
            <div class="shell">
                <SectionHead
                    index="02.4"
                    eyebrow="The Invisible Backbone"
                    title="全球规模"
                    lede="美丽的世界若服务器崩溃便毫无意义。我们构建高度并发、内存安全的后端服务与永不眠的匹配舰队。"
                />
                <div class="infra-grid">
                    {INFRA
                        .iter()
                        .map(|card| {
                            view! {
                                <article class=format!(
                                    "infra-card {}",
                                    if card.wide { "infra-card--wide" } else { "" },
                                )>
                                    <div class="infra-card__head">
                                        <h3 class="infra-card__title">{card.title}</h3>
                                        <span class="infra-card__metric mono">{card.metric}</span>
                                    </div>
                                    <p class="infra-card__text">{card.body}</p>
                                    <div class="tag-row">
                                        {card
                                            .tags
                                            .iter()
                                            .map(|tag| view! { <Tag label=*tag/> })
                                            .collect_view()}
                                    </div>
                                </article>
                            }
                        })
                        .collect_view()}
                </div>
            </div>
        </section>
    }
}

/// 美学愿景
#[component]
fn AestheticsSection() -> impl IntoView {
    view! {
        <section id="aesthetics" class="section section--ink">
            <div class="shell">
                <SectionHead
                    index="02.5"
                    eyebrow="Aesthetic Paradigm"
                    title="美丽怪诞"
                    lede="我们是一个年轻优雅的集体，被不安与超现实所吸引。世界诞生于复古未来主义乌托邦与互联网时代阈限空间的交汇处。"
                />
                <div class="aesthetic-grid">
                    {AESTHETICS
                        .iter()
                        .map(|item| {
                            view! {
                                <article class=format!("aesthetic-card aesthetic-card--{}", item.tone)>
                                    <span class="mono aesthetic-card__index">{item.index}</span>
                                    <h3 class="aesthetic-card__title">{item.title}</h3>
                                    <p class="aesthetic-card__text">{item.body}</p>
                                    <div class="tag-row">
                                        {item
                                            .tags
                                            .iter()
                                            .map(|tag| view! { <Tag label=*tag/> })
                                            .collect_view()}
                                    </div>
                                </article>
                            }
                        })
                        .collect_view()}
                </div>
            </div>
        </section>
    }
}
