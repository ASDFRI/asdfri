pub mod dashboard;
pub mod game;
pub mod hero;
pub mod member;

use api::{
    auth::{Login, Logout},
    user::get_current_user,
};
use leptos::prelude::*;
use leptos_router::components::{A, Outlet};
use ui::{Button, ButtonVariant, Dialog, Field, Snackbar, SnackbarState, Spinner, ToastKind};

/// 控制台上下文中的认证状态
pub type AuthSignal = Signal<bool>;

/// 控制台外壳
#[component]
pub fn Console() -> impl IntoView {
    // 1) 注入全局提示状态
    SnackbarState::provide();

    // 2) 读取当前会话，并允许本地登录/注销覆盖
    let current_user = Resource::new(|| (), |_| get_current_user());
    let auth_override = RwSignal::new(None::<bool>);
    let is_authenticated = Signal::derive(move || {
        auth_override
            .get()
            .unwrap_or_else(|| current_user.get().is_some_and(|result| result.is_ok()))
    });
    provide_context(is_authenticated);

    // 3) 浮层开关
    let login_open = RwSignal::new(false);
    let logout_open = RwSignal::new(false);

    view! {
        <Suspense fallback=move || {
            view! {
                <div class="console-shell">
                    <div class="console-loading">
                        <span class="mono">"LOADING SESSION"</span>
                    </div>
                </div>
            }
        }>
            <div class="console-shell">
                <ConsoleSidebar/>
                <div class="console-main">
                    <ConsoleTopbar
                        is_authenticated=is_authenticated
                        on_login=Callback::new(move |_| login_open.set(true))
                        on_logout=Callback::new(move |_| logout_open.set(true))
                    />
                    <main class="console-content">
                        <Outlet/>
                    </main>
                </div>
            </div>

            <LoginDialog
                open=login_open
                on_success=Callback::new(move |_| auth_override.set(Some(true)))
            />
            <LogoutDialog
                open=logout_open
                on_success=Callback::new(move |_| auth_override.set(Some(false)))
            />
            <Snackbar/>
        </Suspense>
    }
}

/// 侧边栏
#[component]
fn ConsoleSidebar() -> impl IntoView {
    view! {
        <aside class="console-sidebar">
            <A href="/" exact=true attr:class="console-brand">
                <span class="console-brand__mark">"A"</span>
                <span class="console-brand__text">
                    <strong>"ASDF"</strong>
                    <span class="mono">"CONTROL"</span>
                </span>
            </A>

            <nav class="console-nav" aria-label="控制台导航">
                <A href="/console" exact=true attr:class="console-nav__item">
                    <span class="mono">"01"</span>
                    <span>"仪表盘"</span>
                </A>
                <A href="/console/hero" exact=true attr:class="console-nav__item">
                    <span class="mono">"02"</span>
                    <span>"Hero 配置"</span>
                </A>
                <A href="/console/games" exact=true attr:class="console-nav__item">
                    <span class="mono">"03"</span>
                    <span>"游戏配置"</span>
                </A>
                <A href="/console/members" exact=true attr:class="console-nav__item">
                    <span class="mono">"04"</span>
                    <span>"成员配置"</span>
                </A>
            </nav>

            <div class="console-sidebar__foot mono">
                <span>"SINGULARIS"</span>
                <span>"v0.1.0"</span>
            </div>
        </aside>
    }
}

/// 顶部栏
#[component]
fn ConsoleTopbar(
    is_authenticated: AuthSignal,
    on_login: Callback<()>,
    on_logout: Callback<()>,
) -> impl IntoView {
    view! {
        <header class="console-topbar">
            <div class="console-topbar__title">
                <span class="mono">"ASDF / CONSOLE"</span>
                <span class="console-topbar__divider"></span>
                <span class="eyebrow">"Content Operations"</span>
            </div>

            <div class="console-topbar__actions">
                <Show
                    when=move || is_authenticated.get()
                    fallback=move || {
                        view! {
                            <span class="auth-chip auth-chip--offline mono">"未认证"</span>
                            <Button
                                variant=ButtonVariant::Solid
                                class="btn--sm"
                                on_click=Callback::new(move |_| on_login.run(()))
                            >
                                "登录"
                            </Button>
                        }
                    }
                >
                    <span class="auth-chip auth-chip--online mono">"已认证"</span>
                    <Button
                        variant=ButtonVariant::Outline
                        class="btn--sm"
                        on_click=Callback::new(move |_| on_logout.run(()))
                    >
                        "退出"
                    </Button>
                </Show>
            </div>
        </header>
    }
}

/// 登录对话框
#[component]
fn LoginDialog(open: RwSignal<bool>, on_success: Callback<()>) -> impl IntoView {
    let action = ServerAction::<Login>::new();
    let username = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let pending = Signal::derive(move || action.pending().get());
    let snackbar = SnackbarState::use_state();

    Effect::new(move || {
        if let Some(result) = action.value().get() {
            match result {
                Ok(()) => {
                    snackbar.show("认证成功，会话已建立。", ToastKind::Success);
                    open.set(false);
                    on_success.run(());
                }
                Err(error) => snackbar.show(format!("认证失败：{error}"), ToastKind::Error),
            }
            action.value().set(None);
        }
    });

    view! {
        <Dialog
            open=open
            title="登录控制台"
            description="使用管理员凭据建立会话。所有写操作都需要有效令牌。"
        >
            <ActionForm action=action>
                <div class="form-stack">
                    <Field label="用户名" name="username" required=true value=username/>
                    <Field
                        label="密码"
                        input_type="password"
                        name="password"
                        required=true
                        value=password
                    />
                    <div class="dialog-actions">
                        <Button
                            variant=ButtonVariant::Ghost
                            on_click=Callback::new(move |_| open.set(false))
                        >
                            "取消"
                        </Button>
                        <Button
                            variant=ButtonVariant::Solid
                            button_type="submit"
                            disabled=pending
                        >
                            <Show
                                when=move || pending.get()
                                fallback=move || view! { "登录" }
                            >
                                <Spinner class="spinner--sm"/>
                                <span>"验证中"</span>
                            </Show>
                        </Button>
                    </div>
                </div>
            </ActionForm>
        </Dialog>
    }
}

/// 注销对话框
#[component]
fn LogoutDialog(open: RwSignal<bool>, on_success: Callback<()>) -> impl IntoView {
    let action = ServerAction::<Logout>::new();
    let pending = Signal::derive(move || action.pending().get());
    let snackbar = SnackbarState::use_state();

    Effect::new(move || {
        if let Some(result) = action.value().get() {
            match result {
                Ok(()) => {
                    snackbar.show("会话已终止。", ToastKind::Success);
                    open.set(false);
                    on_success.run(());
                }
                Err(error) => snackbar.show(format!("注销失败：{error}"), ToastKind::Error),
            }
            action.value().set(None);
        }
    });

    view! {
        <Dialog
            open=open
            title="退出控制台"
            description="确定要终止当前会话吗？未提交的输入将会丢失。"
        >
            <ActionForm action=action>
                <div class="dialog-actions">
                    <Button
                        variant=ButtonVariant::Ghost
                        on_click=Callback::new(move |_| open.set(false))
                    >
                        "取消"
                    </Button>
                    <Button
                        variant=ButtonVariant::Danger
                        button_type="submit"
                        disabled=pending
                    >
                        <Show
                            when=move || pending.get()
                            fallback=move || view! { "确认退出" }
                        >
                            <Spinner class="spinner--sm"/>
                            <span>"处理中"</span>
                        </Show>
                    </Button>
                </div>
            </ActionForm>
        </Dialog>
    }
}
