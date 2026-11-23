use leptos::prelude::*;

/// 单行文本输入框
#[component]
pub fn Field(
    /// 字段标签
    #[prop(into)]
    label: String,
    /// 原生输入类型
    #[prop(into, default = "text".to_string())]
    input_type: String,
    /// 双向绑定的值
    value: RwSignal<String>,
    /// 原生 id
    #[prop(into, optional)]
    id: Option<String>,
    /// 原生 name
    #[prop(into, optional)]
    name: Option<String>,
    /// 是否必填
    #[prop(optional)]
    required: bool,
    /// 是否禁用
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// 占位提示
    #[prop(into, optional)]
    placeholder: Option<String>,
    /// 附加样式类
    #[prop(into, optional)]
    class: String,
) -> impl IntoView {
    view! {
        <label class=format!("field {}", class)>
            <span class="field__label">{label}</span>
            <input
                type=input_type
                id=id
                name=name
                required=required
                disabled=disabled
                class="field__input"
                placeholder=placeholder.unwrap_or_default()
                prop:value=value
                on:input=move |event| value.set(event_target_value(&event))
            />
        </label>
    }
}

/// 多行文本域
#[component]
pub fn Textarea(
    /// 字段标签
    #[prop(into)]
    label: String,
    /// 双向绑定的值
    value: RwSignal<String>,
    /// 原生 id
    #[prop(into, optional)]
    id: Option<String>,
    /// 原生 name
    #[prop(into, optional)]
    name: Option<String>,
    /// 是否必填
    #[prop(optional)]
    required: bool,
    /// 是否禁用
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// 可见行数
    #[prop(default = 4)]
    rows: usize,
    /// 附加样式类
    #[prop(into, optional)]
    class: String,
) -> impl IntoView {
    view! {
        <label class=format!("field {}", class)>
            <span class="field__label">{label}</span>
            <textarea
                id=id
                name=name
                required=required
                disabled=disabled
                rows=rows
                class="field__input field__input--area"
                prop:value=value
                on:input=move |event| value.set(event_target_value(&event))
            ></textarea>
        </label>
    }
}
