use api::{
    member::{
        CreateMember, DeleteMember, Member, MemberPayload, UpdateMember, UpdateMemberOrders,
        get_members,
    },
    upload::upload_file,
};
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_meta::Title;
use ui::{
    Button, ButtonVariant, Dialog, Field, IconButton, Skeleton, SnackbarState, Spinner, Tag,
    Textarea, ToastKind,
};

use super::AuthSignal;

/// 成员配置页
#[component]
pub fn MemberAdmin() -> impl IntoView {
    let members = Resource::new(|| (), |_| get_members());
    let is_authenticated = use_context::<AuthSignal>().unwrap_or_else(|| Signal::derive(|| false));
    let snackbar = SnackbarState::use_state();

    let edit_open = RwSignal::new(false);
    let edit_target = RwSignal::new(None::<Member>);
    let delete_open = RwSignal::new(false);
    let delete_target = RwSignal::new(None::<i64>);
    let order_action = ServerAction::<UpdateMemberOrders>::new();

    let refresh = Callback::new(move |_| members.refetch());

    let on_create = Callback::new(move |_| {
        edit_target.set(None);
        edit_open.set(true);
    });
    let on_edit = Callback::new(move |member: Member| {
        edit_target.set(Some(member));
        edit_open.set(true);
    });
    let on_delete = Callback::new(move |id: i64| {
        delete_target.set(Some(id));
        delete_open.set(true);
    });

    let on_move = Callback::new(move |(id, offset): (i64, isize)| {
        let Some(Ok(mut items)) = members.get() else {
            return;
        };

        items.sort_by(|left, right| {
            left.sort_order.cmp(&right.sort_order).then_with(|| right.id.cmp(&left.id))
        });

        let Some(index) = items.iter().position(|member| member.id == id) else {
            return;
        };
        let target = index as isize + offset;
        if target < 0 || target as usize >= items.len() {
            return;
        }

        items.swap(index, target as usize);
        let orders =
            items.iter().enumerate().map(|(index, member)| (member.id, index as i64)).collect();

        order_action.dispatch(UpdateMemberOrders {
            orders,
        });
    });
    let on_move_up = Callback::new(move |id| on_move.run((id, -1)));
    let on_move_down = Callback::new(move |id| on_move.run((id, 1)));

    Effect::new(move || {
        if let Some(result) = order_action.value().get() {
            match result {
                Ok(()) => {
                    snackbar.show("展示顺序已同步。", ToastKind::Success);
                    members.refetch();
                }
                Err(error) => snackbar.show(format!("排序失败：{error}"), ToastKind::Error),
            }
            order_action.value().set(None);
        }
    });

    view! {
        <Title text="Members — ASDF Console"/>
        <header class="console-page-head">
            <div>
                <span class="eyebrow">"Organization"</span>
                <h1 class="console-page-title">"成员配置"</h1>
                <p class="console-page-lede">
                    "维护团队成员的身份信息、头像与展示顺序。"
                </p>
            </div>
            <div class="console-page-head__actions">
                <span class="mono console-page-code">"04 / MEMBERS"</span>
                <Button
                    variant=ButtonVariant::Solid
                    disabled=Signal::derive(move || !is_authenticated.get())
                    on_click=on_create
                >
                    "新增成员"
                </Button>
            </div>
        </header>

        <Show when=move || !is_authenticated.get()>
            <div class="lock-panel">
                <span class="mono">"READ ONLY"</span>
                <p>"当前会话未认证：可以浏览档案，但新增、编辑、删除与排序都会被服务端拒绝。"</p>
            </div>
        </Show>

        <Suspense fallback=move || {
            view! {
                <div class="admin-list">
                    <Skeleton class="admin-row admin-row--skeleton"/>
                    <Skeleton class="admin-row admin-row--skeleton"/>
                    <Skeleton class="admin-row admin-row--skeleton"/>
                </div>
            }
        }>
            {move || {
                match members.get() {
                    Some(Ok(items)) if items.is_empty() => {
                        view! {
                            <div class="empty-state">
                                <span class="mono">"EMPTY"</span>
                                <h2>"成员档案为空"</h2>
                                <p>"通过右上角的新增按钮录入第一条成员记录。"</p>
                            </div>
                        }
                            .into_any()
                    }
                    Some(Ok(items)) => {
                        let len = items.len();
                        view! {
                            <div class="admin-list">
                                {items
                                    .into_iter()
                                    .enumerate()
                                    .map(|(index, member)| {
                                        view! {
                                            <MemberRow
                                                member=member
                                                position=index
                                                can_move_up={index > 0}
                                                can_move_down={index + 1 < len}
                                                on_edit=on_edit
                                                on_delete=on_delete
                                                on_move_up=on_move_up
                                                on_move_down=on_move_down
                                            />
                                        }
                                    })
                                    .collect_view()}
                            </div>
                        }
                            .into_any()
                    }
                    Some(Err(error)) => {
                        view! {
                            <p class="notice notice--error">{format!("读取失败：{error}")}</p>
                        }
                            .into_any()
                    }
                    None => ().into_any(),
                }
            }}
        </Suspense>

        <MemberEditDialog
            open=edit_open
            initial=edit_target
            on_success=refresh
        />
        <MemberDeleteDialog
            open=delete_open
            target=delete_target
            on_success=refresh
        />
    }
}

/// 单行成员记录
#[component]
fn MemberRow(
    member: Member,
    position: usize,
    can_move_up: bool,
    can_move_down: bool,
    on_edit: Callback<Member>,
    on_delete: Callback<i64>,
    on_move_up: Callback<i64>,
    on_move_down: Callback<i64>,
) -> impl IntoView {
    let Member {
        id,
        name,
        icon,
        description,
        slogan,
        tags,
        position: role,
        sort_order,
    } = member.clone();
    let avatar = icon.unwrap_or_else(|| "/ASDF.png".to_string());
    let avatar_signal = RwSignal::new(avatar);
    let edit_target = member;

    view! {
        <article class="admin-row">
            <span class="admin-row__index mono">{format!("{:02}", position + 1)}</span>
            <div class="admin-row__media admin-row__media--portrait">
                <img
                    src=move || avatar_signal.get()
                    on:error=move |_| avatar_signal.set("/ASDF.png".to_string())
                    alt=name.clone()
                    loading="lazy"
                />
            </div>
            <div class="admin-row__body">
                <h2 class="admin-row__title">{name.clone()}</h2>
                <p class="admin-row__text">
                    {slogan
                        .or(description)
                        .unwrap_or_else(|| "暂无签名或描述。".to_string())}
                </p>
                <div class="tag-row">
                    {tags.into_iter().map(|tag| view! { <Tag label=tag/> }).collect_view()}
                </div>
            </div>
            <div class="admin-row__meta mono">
                <span>{format!("SORT {sort_order:03}")}</span>
                {role.map(|value| view! { <span>{value}</span> })}
            </div>
            <div class="admin-row__actions">
                <IconButton
                    label="上移"
                    disabled=!can_move_up
                    on_click=Callback::new(move |_| on_move_up.run(id))
                >
                    "↑"
                </IconButton>
                <IconButton
                    label="下移"
                    disabled=!can_move_down
                    on_click=Callback::new(move |_| on_move_down.run(id))
                >
                    "↓"
                </IconButton>
                <IconButton
                    label="编辑"
                    on_click=Callback::new(move |_| on_edit.run(edit_target.clone()))
                >
                    "编辑"
                </IconButton>
                <IconButton
                    label="删除"
                    on_click=Callback::new(move |_| on_delete.run(id))
                >
                    "删除"
                </IconButton>
            </div>
        </article>
    }
}

/// 成员编辑对话框
#[component]
fn MemberEditDialog(
    open: RwSignal<bool>,
    initial: RwSignal<Option<Member>>,
    on_success: Callback<()>,
) -> impl IntoView {
    let name = RwSignal::new(String::new());
    let icon = RwSignal::new(String::new());
    let position = RwSignal::new(String::new());
    let slogan = RwSignal::new(String::new());
    let description = RwSignal::new(String::new());
    let tags_text = RwSignal::new(String::new());
    let create_action = ServerAction::<CreateMember>::new();
    let update_action = ServerAction::<UpdateMember>::new();
    let uploading = RwSignal::new(false);
    let pending = Signal::derive(move || {
        create_action.pending().get() || update_action.pending().get() || uploading.get()
    });
    let snackbar = SnackbarState::use_state();
    let is_edit = Signal::derive(move || initial.get().is_some());

    Effect::new(move || {
        if !open.get() {
            return;
        }

        match initial.get() {
            Some(member) => {
                name.set(member.name);
                icon.set(member.icon.unwrap_or_default());
                position.set(member.position.unwrap_or_default());
                slogan.set(member.slogan.unwrap_or_default());
                description.set(member.description.unwrap_or_default());
                tags_text.set(member.tags.join(", "));
            }
            None => {
                name.set(String::new());
                icon.set(String::new());
                position.set(String::new());
                slogan.set(String::new());
                description.set(String::new());
                tags_text.set(String::new());
            }
        }
    });

    Effect::new(move || {
        if let Some(result) = create_action.value().get() {
            match result {
                Ok(_) => {
                    snackbar.show("成员档案已创建。", ToastKind::Success);
                    open.set(false);
                    on_success.run(());
                }
                Err(error) => snackbar.show(format!("创建失败：{error}"), ToastKind::Error),
            }
            create_action.value().set(None);
        }

        if let Some(result) = update_action.value().get() {
            match result {
                Ok(()) => {
                    snackbar.show("成员档案已更新。", ToastKind::Success);
                    open.set(false);
                    on_success.run(());
                }
                Err(error) => snackbar.show(format!("更新失败：{error}"), ToastKind::Error),
            }
            update_action.value().set(None);
        }
    });

    let on_file = move |event: leptos::ev::Event| {
        let input: web_sys::HtmlInputElement = event_target(&event);
        let Some(files) = input.files() else {
            return;
        };
        let Some(file) = files.get(0) else {
            return;
        };

        let form = match web_sys::FormData::new() {
            Ok(form) => form,
            Err(_) => {
                snackbar.show("无法构造上传表单。", ToastKind::Error);
                return;
            }
        };
        if form.append_with_blob("file", &file).is_err() {
            snackbar.show("无法读取所选文件。", ToastKind::Error);
            return;
        }

        uploading.set(true);
        spawn_local(async move {
            match upload_file(form.into()).await {
                Ok(response) if response.success => {
                    if let Some(url) = response.url {
                        icon.set(url);
                        snackbar.show("头像上传成功。", ToastKind::Success);
                    }
                }
                Ok(_) => snackbar.show("服务端拒绝了该文件。", ToastKind::Error),
                Err(error) => snackbar.show(format!("上传失败：{error}"), ToastKind::Error),
            }
            uploading.set(false);
        });

        input.set_value("");
    };

    let submit = move || {
        if name.get().trim().is_empty() {
            snackbar.show("成员姓名不能为空。", ToastKind::Error);
            return;
        }

        let optional = |value: String| {
            let value = value.trim().to_string();
            (!value.is_empty()).then_some(value)
        };
        let tags = tags_text
            .get()
            .split(',')
            .map(str::trim)
            .filter(|tag| !tag.is_empty())
            .map(str::to_string)
            .collect::<Vec<_>>();

        let payload = MemberPayload {
            name: name.get(),
            icon: optional(icon.get()),
            description: optional(description.get()),
            slogan: optional(slogan.get()),
            tags,
            position: optional(position.get()),
            sort_order: initial.get().map(|member| member.sort_order),
        };

        match initial.get() {
            Some(member) => update_action.dispatch(UpdateMember {
                id: member.id,
                payload,
            }),
            None => create_action.dispatch(CreateMember {
                payload,
            }),
        };
    };

    view! {
        <Dialog
            open=open
            title="成员档案"
            description="录入或更新成员身份信息。头像仅接受由服务端识别的图片格式。"
        >
            <form
                class="form-stack"
                on:submit=move |event| {
                    event.prevent_default();
                    submit();
                }
            >
                <div class="avatar-field">
                    <div class="avatar-field__preview">
                        <img src=move || {
                            let value = icon.get();
                            if value.is_empty() { "/ASDF.png".to_string() } else { value }
                        } alt="头像预览"/>
                    </div>
                    <div class="avatar-field__controls">
                        <span class="field__label">"视觉标识"</span>
                        <label class="upload__drop upload__drop--compact">
                            <Show
                                when=move || uploading.get()
                                fallback=move || view! { <span class="mono">"上传头像"</span> }
                            >
                                <Spinner class="spinner--sm"/>
                                <span class="mono">"上传中"</span>
                            </Show>
                            <input
                                type="file"
                                class="visually-hidden"
                                accept="image/*"
                                on:change=on_file
                                disabled=move || uploading.get()
                            />
                        </label>
                    </div>
                </div>

                <Field label="成员姓名 *" name="name" required=true value=name/>
                <Field label="组织职位" value=position/>
                <Field label="个性签名" value=slogan/>
                <Field label="领域标签（英文逗号分隔）" value=tags_text/>
                <Textarea label="详细描述" value=description rows=3/>

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
                            fallback=move || {
                                view! {
                                    {move || if is_edit.get() { "保存更改" } else { "创建档案" }}
                                }
                            }
                        >
                            <Spinner class="spinner--sm"/>
                            <span>"写入中"</span>
                        </Show>
                    </Button>
                </div>
            </form>
        </Dialog>
    }
}

/// 成员删除确认对话框
#[component]
fn MemberDeleteDialog(
    open: RwSignal<bool>,
    target: RwSignal<Option<i64>>,
    on_success: Callback<()>,
) -> impl IntoView {
    let action = ServerAction::<DeleteMember>::new();
    let pending = Signal::derive(move || action.pending().get());
    let snackbar = SnackbarState::use_state();

    Effect::new(move || {
        if let Some(result) = action.value().get() {
            match result {
                Ok(()) => {
                    snackbar.show("成员档案已删除。", ToastKind::Success);
                    open.set(false);
                    target.set(None);
                    on_success.run(());
                }
                Err(error) => snackbar.show(format!("删除失败：{error}"), ToastKind::Error),
            }
            action.value().set(None);
        }
    });

    view! {
        <Dialog
            open=open
            title="删除成员档案"
            description="该操作不可逆。成员记录与关联的头像地址将被永久移除。"
        >
            <div class="dialog-actions">
                <Button
                    variant=ButtonVariant::Ghost
                    on_click=Callback::new(move |_| open.set(false))
                >
                    "取消"
                </Button>
                <Button
                    variant=ButtonVariant::Danger
                    disabled=pending
                    on_click=Callback::new(move |_| {
                        if let Some(id) = target.get() {
                            action.dispatch(DeleteMember { id });
                        }
                    })
                >
                    <Show
                        when=move || pending.get()
                        fallback=move || view! { "确认删除" }
                    >
                        <Spinner class="spinner--sm"/>
                        <span>"删除中"</span>
                    </Show>
                </Button>
            </div>
        </Dialog>
    }
}
