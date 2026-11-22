use leptos::prelude::*;
use leptos_meta::{Meta, Title};
use ui::Tag;

use crate::components::SectionHead;

/// 架构特性
struct ArchSpec {
    index: &'static str,
    title: &'static str,
    body: &'static str,
    tags: &'static [&'static str],
    wide: bool,
}

/// 模块条目
struct ModuleSpec {
    name: &'static str,
    status: &'static str,
    version: &'static str,
    body: &'static str,
    tags: &'static [&'static str],
    tone: &'static str,
}

const ARCHITECTURE: [ArchSpec; 4] = [
    ArchSpec {
        index: "01",
        title: "非侵入式设计",
        body: "按需所用，弃留自如。零紧耦合，结构自由无拘。SingularisInteraction 不需要知道 SingularisInventory 的存在。",
        tags: &["解耦", "单一职责"],
        wide: true,
    },
    ArchSpec {
        index: "02",
        title: "网络复制",
        body: "所有插件在设计时都充分考虑网络复制，原生支持多人游戏。SingularisOnline 开箱即用地处理 Lobby 与会话。",
        tags: &["Replication", "Session"],
        wide: false,
    },
    ArchSpec {
        index: "03",
        title: "蓝图友好",
        body: "高性能 C++ 基础架构完美兼容蓝图。让设计师高效工作，而无需牺牲哪怕一毫秒的帧时间。",
        tags: &["C++", "Blueprints"],
        wide: false,
    },
    ArchSpec {
        index: "04",
        title: "严格命名 & 惯例",
        body: "混乱的项目注定失败。自动化工具与编辑器实用程序确保资源、类与结构井然有序。",
        tags: &["Automation", "Conventions"],
        wide: true,
    },
];

const MODULES: [ModuleSpec; 6] = [
    ModuleSpec {
        name: "SingularisCore",
        status: "FOUNDATION",
        version: "v2.1.0",
        body: "核心基础架构层。定义全局统一接口规范、共享数据类型体系、子系统管理机制，以及一组经过内存优化的实用宏。",
        tags: &["Core", "Types", "Macro", "Subsystems"],
        tone: "blue",
    },
    ModuleSpec {
        name: "SingularisInteraction",
        status: "PRODUCTION",
        version: "v1.4.2",
        body: "高度解耦的基于轨迹的交互系统。支持上下文提示、基于时长的保持，以及复杂的多参与者状态变更。",
        tags: &["Replicated", "Trace Profiles"],
        tone: "green",
    },
    ModuleSpec {
        name: "SingularisInventory",
        status: "PRODUCTION",
        version: "v1.3.0",
        body: "基于槽位与网格的库存架构。支持用于复制的快速数组、负重机制，以及无缝的装备更换。",
        tags: &["Fast Array RPC", "DataAssets"],
        tone: "green",
    },
    ModuleSpec {
        name: "SingularisVehicle",
        status: "BETA",
        version: "v0.9.5",
        body: "基于 Chaos Physics 的先进物理驱动车辆框架，支持模块化部件更换、多层次损伤模型与服务端权威运动。",
        tags: &["Chaos Physics", "Prediction"],
        tone: "amber",
    },
    ModuleSpec {
        name: "SingularisBuild",
        status: "PRODUCTION",
        version: "v1.1.0",
        body: "融合网格吸附与自由形态的建造机制，支持结构完整性计算、基于插槽的连接规则以及状态保存序列化。",
        tags: &["Serialization", "Socket Logic"],
        tone: "green",
    },
    ModuleSpec {
        name: "SingularisOnline",
        status: "EVOLVING",
        version: "v1.0.2",
        body: "与 EOS 和 Steam 无缝集成，为跨平台联机提供完整支持：会话管理、专有服务器握手、匹配与玩家持久化。",
        tags: &["EOS SDK", "Session API"],
        tone: "violet",
    },
];

const TOOLCHAIN: [(&str, &str); 3] = [
    (
        "资产命名检查器",
        "自动扫描内容浏览器。强制执行前缀规则（BP_、WBP_、MI_）并根据 Singularis 规范验证目录结构。",
    ),
    ("依赖图验证器", "检测并阻止蓝图模块之间的循环依赖，防止其导致打包失败或运行时崩溃。"),
    (
        "C++ 样板代码生成器",
        "一键生成新的子系统、快速数组与复制组件代码框架，并预先配置 Singularis 日志宏。",
    ),
];

/// Singularis 页
#[component]
pub fn SingularisPage() -> impl IntoView {
    view! {
        <Title text="Singularis — ASDF Studio"/>
        <Meta
            name="description"
            content="Singularis：面向虚幻引擎 5 的模块化游戏框架，包含 15 个以上生产可用的插件与工具。"
        />
        <section class="page-hero">
            <div class="shell page-hero__grid">
                <span class="index-mark">"05"</span>
                <div class="page-hero__text">
                    <span class="eyebrow">"Unreal Engine 5 Ecosystem"</span>
                    <h1 class="display">"引力奇点"</h1>
                    <p class="lede">
                        "面向下一代交互体验的终极模块化框架。包含 15 个以上可直接用于生产环境的插件与工具的统一架构。"
                    </p>
                    <div class="page-hero__actions">
                        <a href="#architecture" class="btn btn--solid">"探索框架"</a>
                        <a href="#manifesto" class="btn btn--outline">"阅读宣言"</a>
                    </div>
                </div>
                <div class="hero-matrix">
                    {[
                        "SingularisCore",
                        "SingularisInteraction",
                        "SingularisVehicle",
                        "SingularisBasium",
                        "SingularisOnline",
                        "SingularisBuild",
                        "SingularisInventory",
                        "SingularisMessage",
                        "SingularisWidget",
                        "SingularisLua",
                    ]
                        .into_iter()
                        .map(|name| view! { <span class="hero-matrix__item mono">{name}</span> })
                        .collect_view()}
                </div>
            </div>
        </section>

        <section id="architecture" class="section">
            <div class="shell">
                <SectionHead
                    index="05.1"
                    eyebrow="System Architecture"
                    title="专为规模化而打造"
                    lede="专为虚幻引擎 5 量身打造。遵循单一职责原则，每个插件严格解耦、互不依赖，无运行时性能损耗，并提供一流的编辑器支持。"
                />
                <div class="arch-grid">
                    {ARCHITECTURE
                        .iter()
                        .map(|item| {
                            view! {
                                <article class=format!(
                                    "arch-card {}",
                                    if item.wide { "arch-card--wide" } else { "" },
                                )>
                                    <span class="mono arch-card__index">{item.index}</span>
                                    <h3 class="arch-card__title">{item.title}</h3>
                                    <p class="arch-card__text">{item.body}</p>
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

        <section id="modules" class="section section--ink">
            <div class="shell">
                <SectionHead
                    index="05.2"
                    eyebrow="The Arsenal"
                    title="核心插件"
                    lede="15+ 活跃模块 · 100% 多人游戏就绪。每个模块都可以独立引入，也可以组合成完整的技术底座。"
                />
                <div class="module-grid">
                    {MODULES
                        .iter()
                        .map(|module| {
                            view! {
                                <article class=format!("module-card module-card--{}", module.tone)>
                                    <div class="module-card__head">
                                        <span class="module-card__status mono">{module.status}</span>
                                        <span class="module-card__version mono">{module.version}</span>
                                    </div>
                                    <h3 class="module-card__name mono">{module.name}</h3>
                                    <p class="module-card__text">{module.body}</p>
                                    <div class="tag-row">
                                        {module
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

        <section id="manifesto" class="section">
            <div class="shell toolchain">
                <div class="toolchain__text">
                    <SectionHead
                        index="05.3"
                        eyebrow="The Singularis Standard"
                        title="Ruthless Automation"
                        lede="缺乏强制力的框架不过是建议。Singularis 配备专用编辑器工具面板与命令行工具，严格监控项目的代码健康。"
                    />
                    <div class="feature-list">
                        {TOOLCHAIN
                            .into_iter()
                            .map(|(title, body)| {
                                view! {
                                    <div class="feature">
                                        <h3 class="feature__title">{title}</h3>
                                        <p class="feature__text">{body}</p>
                                    </div>
                                }
                            })
                            .collect_view()}
                    </div>
                </div>

                <div class="terminal">
                    <div class="terminal__bar">
                        <span class="mono">"Singularis Validator // Run_Pass_01"</span>
                    </div>
                    <div class="terminal__body mono">
                        <p><span class="terminal__tag terminal__tag--info">"[INFO]"</span>" Initializing Project Scan..."</p>
                        <p><span class="terminal__tag">"[12:04:01]"</span>" Scanning directory: /Game/Core/Weapons/"</p>
                        <p><span class="terminal__tag terminal__tag--pass">"[PASS]"</span>" BP_AssaultRifle_Singularis matches prefix convention."</p>
                        <p><span class="terminal__tag terminal__tag--pass">"[PASS]"</span>" SK_AssaultRifle_Mesh validated."</p>
                        <p class="terminal__row terminal__row--fail">
                            <span class="terminal__tag terminal__tag--fail">"[FAIL]"</span>
                            <span>
                                "Invalid Asset Name detected: 'Weapon_Pistol_New'"
                                <br/>
                                <span class="terminal__hint">"-> Expected prefix: 'BP_'. Auto-fix available."</span>
                            </span>
                        </p>
                        <p><span class="terminal__tag terminal__tag--exec">"[EXEC]"</span>" Applying Singularis Standard renaming rules..."</p>
                        <p><span class="terminal__tag terminal__tag--pass">"[OK]"</span>" Renamed to: BP_WeaponPistol"</p>
                        <p class="terminal__prompt">
                            <span class="terminal__tag terminal__tag--info">"user@singularis:~$"</span>
                            <span class="terminal__cursor"></span>
                        </p>
                    </div>
                </div>
            </div>
        </section>

        <section id="access" class="section section--ink">
            <div class="shell cta-band">
                <div>
                    <span class="eyebrow">"System Ready"</span>
                    <h2 class="display cta-band__title">"初始化你的核心"</h2>
                    <p class="lede">
                        "告别在每个新项目中重复编写库存与交互逻辑。采用 Singularis 框架，即刻开始真正的游戏构建。"
                    </p>
                    <code class="command-line mono">"git clone https://github.com/ASDFStudio/Singularis.git"</code>
                </div>
                <div class="cta-band__actions">
                    <a href="#modules" class="btn btn--accent">"阅读文档"</a>
                    <a
                        href="https://github.com/ASDFStudio/Singularis"
                        target="_blank"
                        rel="noopener noreferrer"
                        class="btn btn--outline"
                    >
                        "GitHub"
                    </a>
                </div>
            </div>
        </section>
    }
}
