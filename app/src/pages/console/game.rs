use api::{
    game::{CreateGame, DeleteGame, Game, GamePayload, UpdateGame, UpdateGameOrders, get_games},
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

/// 游戏配置页
#[component]
pub fn GameAdmin() -> impl IntoView {
    let games = Resource::new(|| (), |_| get_games());
    let is_authenticated = use_context::<AuthSignal>().unwrap_or_else(|| Signal::derive(|| false));
    let snackbar = SnackbarState::use_state();

    let edit_open = RwSignal::new(false);
    let edit_target = RwSignal::new(None::<Game>);
    let delete_open = RwSignal::new(false);
    let delete_target = RwSignal::new(None::<i64>);
    let order_action = ServerAction::<UpdateGameOrders>::new();

    let refresh = Callback::new(move |_| games.refetch());

    let on_create = Callback::new(move |_| {
        edit_target.set(None);
        edit_open.set(true);
    });
    let on_edit = Callback::new(move |game: Game| {
        edit_target.set(Some(game));
        edit_open.set(true);
    });
    let on_delete = Callback::new(move |id: i64| {
        delete_target.set(Some(id));
        delete_open.set(true);
    });

    let on_move = Callback::new(move |(id, offset): (i64, isize)| {
        let Some(Ok(mut items)) = games.get() else {
            return;
        };

        items.sort_by(|left, right| {
            left.sort_order.cmp(&right.sort_order).then_with(|| right.id.cmp(&left.id))
        });

        let Some(index) = items.iter().position(|game| game.id == id) else {
            return;
        };
        let target = index as isize + offset;
        if target < 0 || target as usize >= items.len() {
            return;
        }

        items.swap(index, target as usize);
        let orders =
            items.iter().enumerate().map(|(index, game)| (game.id, index as i64)).collect();

        order_action.dispatch(UpdateGameOrders {
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
                    games.refetch();
                }
                Err(error) => snackbar.show(format!("排序失败：{error}"), ToastKind::Error),
            }
            order_action.value().set(None);
        }
    });

    view! {
        <Title text="Games — ASDF Console"/>
        <header class="console-page-head">
            <div>
                <span class="eyebrow">"Content Library"</span>
                <h1 class="console-page-title">"游戏配置"</h1>
                <p class="console-page-lede">
                    "维护作品元数据、截图资产与展示顺序。上移/下移会立即写回数据库。"
                </p>
            </div>
            <div class="console-page-head__actions">
                <span class="mono console-page-code">"03 / GAMES"</span>
                <Button
                    variant=ButtonVariant::Solid
                    disabled=Signal::derive(move || !is_authenticated.get())
                    on_click=on_create
                >
                    "新增游戏"
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
                match games.get() {
                    Some(Ok(items)) if items.is_empty() => {
                        view! {
                            <div class="empty-state">
                                <span class="mono">"EMPTY"</span>
                                <h2>"游戏档案为空"</h2>
                                <p>"通过右上角的新增按钮录入第一条作品记录。"</p>
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
                                    .map(|(index, game)| {
                                        view! {
                                            <GameRow
                                                game=game
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

        <GameEditDialog
            open=edit_open
            initial=edit_target
            on_success=refresh
        />
        <GameDeleteDialog
            open=delete_open
            target=delete_target
            on_success=refresh
        />
    }
}

/// 单行游戏记录
#[component]
fn GameRow(
    game: Game,
    position: usize,
    can_move_up: bool,
    can_move_down: bool,
    on_edit: Callback<Game>,
    on_delete: Callback<i64>,
    on_move_up: Callback<i64>,
    on_move_down: Callback<i64>,
) -> impl IntoView {
    let Game {
        id,
        name,
        screenshots,
        description,
        tags,
        link,
        sort_order,
    } = game.clone();
    let cover = screenshots.first().cloned().unwrap_or_else(|| "/ASDF.png".to_string());
    let cover_signal = RwSignal::new(cover);
    let edit_target = game;

    view! {
        <article class="admin-row">
            <span class="admin-row__index mono">{format!("{:02}", position + 1)}</span>
            <div class="admin-row__media">
                <img
                    src=move || cover_signal.get()
                    on:error=move |_| cover_signal.set("/ASDF.png".to_string())
                    alt=name.clone()
                    loading="lazy"
                />
            </div>
            <div class="admin-row__body">
                <h2 class="admin-row__title">{name.clone()}</h2>
                <p class="admin-row__text">
                    {description.unwrap_or_else(|| "暂无描述。".to_string())}
                </p>
                <div class="tag-row">
                    {tags.into_iter().map(|tag| view! { <Tag label=tag/> }).collect_view()}
                </div>
            </div>
            <div class="admin-row__meta mono">
                <span>{format!("SORT {sort_order:03}")}</span>
                {link.map(|href| {
                    view! {
                        <a href=href target="_blank" rel="noopener noreferrer">"LINK ↗"</a>
                    }
                })}
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

/// 游戏编辑对话框
#[component]
fn GameEditDialog(
    open: RwSignal<bool>,
    initial: RwSignal<Option<Game>>,
    on_success: Callback<()>,
) -> impl IntoView {
    let name = RwSignal::new(String::new());
    let description = RwSignal::new(String::new());
    let tags_text = RwSignal::new(String::new());
    let link = RwSignal::new(String::new());
    let screenshots = RwSignal::new(Vec::<String>::new());
    let create_action = ServerAction::<CreateGame>::new();
    let update_action = ServerAction::<UpdateGame>::new();
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
            Some(game) => {
                name.set(game.name);
                description.set(game.description.unwrap_or_default());
                tags_text.set(game.tags.join(", "));
                link.set(game.link.unwrap_or_default());
                screenshots.set(game.screenshots);
            }
            None => {
                name.set(String::new());
                description.set(String::new());
                tags_text.set(String::new());
                link.set(String::new());
                screenshots.set(Vec::new());
            }
        }
    });

    Effect::new(move || {
        if let Some(result) = create_action.value().get() {
            match result {
                Ok(_) => {
                    snackbar.show("游戏档案已创建。", ToastKind::Success);
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
                    snackbar.show("游戏档案已更新。", ToastKind::Success);
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
                        screenshots.update(|list| list.push(url));
                        snackbar.show("截图上传成功。", ToastKind::Success);
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
            snackbar.show("游戏名称不能为空。", ToastKind::Error);
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

        let payload = GamePayload {
            name: name.get(),
            screenshots: screenshots.get(),
            description: optional(description.get()),
            tags,
            link: optional(link.get()),
            sort_order: initial.get().map(|game| game.sort_order),
        };

        match initial.get() {
            Some(game) => update_action.dispatch(UpdateGame {
                id: game.id,
                payload,
            }),
            None => create_action.dispatch(CreateGame {
                payload,
            }),
        };
    };

    view! {
        <Dialog
            open=open
            title="游戏档案"
            description="录入或更新作品元数据。截图仅接受由服务端识别的图片格式。"
        >
            <form
                class="form-stack"
                on:submit=move |event| {
                    event.prevent_default();
                    submit();
                }
            >
                <Field label="游戏名称 *" name="name" required=true value=name/>
                <Textarea label="游戏简介" value=description rows=4/>
                <Field label="分类标签（英文逗号分隔）" value=tags_text/>
                <Field label="外部链接" value=link/>

                <div class="upload">
                    <span class="field__label">"截图资产"</span>
                    <div class="upload__grid">
                        <For
                            each=move || screenshots.get()
                            key=|url| url.clone()
                            children=move |url| {
                                let remove_target = url.clone();
                                view! {
                                    <figure class="upload__item">
                                        <img src=url alt="截图预览"/>
                                        <button
                                            type="button"
                                            class="upload__remove mono"
                                            aria-label="移除截图"
                                            on:click=move |_| {
                                                screenshots
                                                    .update(|list| list.retain(|item| item != &remove_target))
                                            }
                                        >
                                            "REMOVE"
                                        </button>
                                    </figure>
                                }
                            }
                        />
                    </div>
                    <label class="upload__drop">
                        <Show
                            when=move || uploading.get()
                            fallback=move || {
                                view! {
                                    <span class="mono">"选择图片上传"</span>
                                    <span class="upload__hint">"PNG / JPEG / GIF / WebP / AVIF · ≤ 16 MiB"</span>
                                }
                            }
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

/// 游戏删除确认对话框
#[component]
fn GameDeleteDialog(
    open: RwSignal<bool>,
    target: RwSignal<Option<i64>>,
    on_success: Callback<()>,
) -> impl IntoView {
    let action = ServerAction::<DeleteGame>::new();
    let pending = Signal::derive(move || action.pending().get());
    let snackbar = SnackbarState::use_state();

    Effect::new(move || {
        if let Some(result) = action.value().get() {
            match result {
                Ok(()) => {
                    snackbar.show("游戏档案已删除。", ToastKind::Success);
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
            title="删除游戏档案"
            description="该操作不可逆。作品记录与关联的截图地址将被永久移除。"
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
                            action.dispatch(DeleteGame { id });
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
