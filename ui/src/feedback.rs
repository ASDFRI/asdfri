use std::time::Duration;

use leptos::prelude::*;

/// 加载指示器
#[component]
pub fn Spinner(#[prop(into, optional)] class: String) -> impl IntoView {
    view! { <span class=format!("spinner {}", class) aria-hidden="true"></span> }
}

/// 骨架占位块
#[component]
pub fn Skeleton(#[prop(into, optional)] class: String) -> impl IntoView {
    view! { <div class=format!("skeleton {}", class) aria-hidden="true"></div> }
}

/// 分割线
#[component]
pub fn Divider(#[prop(into, optional)] class: String) -> impl IntoView {
    view! { <hr class=format!("rule {}", class)/> }
}

/// 线性进度条
#[component]
pub fn ProgressBar(
    /// 0–100 的进度值
    #[prop(into)]
    value: Signal<f64>,
    /// 附加样式类
    #[prop(into, optional)]
    class: String,
) -> impl IntoView {
    view! {
        <div class=format!("progress {}", class) role="progressbar">
            <div
                class="progress__fill"
                style:width=move || format!("{:.2}%", value.get().clamp(0.0, 100.0))
            ></div>
        </div>
    }
}

/// 提示消息类型
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToastKind {
    /// 成功
    Success,
    /// 失败
    Error,
    /// 一般信息
    Info,
}

impl ToastKind {
    /// 对应的样式类
    fn class(self) -> &'static str {
        match self {
            Self::Success => "toast--success",
            Self::Error => "toast--error",
            Self::Info => "toast--info",
        }
    }
}

/// 单条提示消息
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Toast {
    /// 文本
    pub text: String,
    /// 类型
    pub kind: ToastKind,
    /// 是否可见
    pub visible: bool,
}

/// 全局提示状态
#[derive(Clone, Copy)]
pub struct SnackbarState {
    /// 当前消息
    pub message: RwSignal<Toast>,
    /// 消息序号，用于防止旧计时器提前隐藏新消息
    sequence: RwSignal<u64>,
}

impl SnackbarState {
    /// 注入提示状态
    pub fn provide() {
        provide_context(Self {
            message: RwSignal::new(Toast {
                text: String::new(),
                kind: ToastKind::Info,
                visible: false,
            }),
            sequence: RwSignal::new(0),
        });
    }

    /// 读取提示状态
    pub fn use_state() -> Self {
        expect_context::<Self>()
    }

    /// 展示一条提示并在数秒后自动收起
    pub fn show(&self, text: impl Into<String>, kind: ToastKind) {
        let message = self.message;
        let sequence = self.sequence;
        let current = sequence.get_untracked().wrapping_add(1);

        sequence.set(current);
        message.set(Toast {
            text: text.into(),
            kind,
            visible: true,
        });
        set_timeout(
            move || {
                if sequence.get_untracked() == current {
                    message.update(|toast| toast.visible = false);
                }
            },
            Duration::from_secs(4),
        );
    }
}

/// 全局提示条
#[component]
pub fn Snackbar() -> impl IntoView {
    let state = SnackbarState::use_state();

    view! {
        <div
            class=move || {
                let toast = state.message.get();
                format!(
                    "toast {} {}",
                    toast.kind.class(),
                    if toast.visible { "is-visible" } else { "" }
                )
            }
            role="status"
            aria-live="polite"
        >
            <span class="toast__mark"></span>
            <span>{move || state.message.get().text}</span>
        </div>
    }
}
