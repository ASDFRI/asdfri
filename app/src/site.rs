//! 站点视觉变体的运行时上下文。

use api::site::SiteVariant;
use leptos::prelude::*;

/// 浏览器地址栏主题色
pub fn theme_color(variant: SiteVariant) -> &'static str {
    match variant {
        SiteVariant::Swiss => "#f3f0e9",
        SiteVariant::Constructivist => "#0b0b0b",
    }
}

/// 站点风格信号
#[derive(Clone, Copy)]
pub struct SiteVariantSignal(RwSignal<SiteVariant>);

impl SiteVariantSignal {
    /// 从 SSR 上下文或页面根节点恢复初始风格并注入上下文
    pub fn provide() -> Self {
        let initial = use_context::<SiteVariant>().or_else(document_variant).unwrap_or_default();
        let this = Self(RwSignal::new(initial));
        provide_context(this);

        this
    }

    /// 读取上下文
    pub fn use_state() -> Self {
        expect_context::<Self>()
    }

    /// 当前风格
    pub fn get(self) -> SiteVariant {
        self.0.get()
    }

    /// 更新风格
    pub fn set(self, variant: SiteVariant) {
        self.0.set(variant);
    }
}

/// 将当前风格写入 `<html data-variant>`，供样式与自动化测试识别
#[cfg(target_arch = "wasm32")]
pub fn sync_document_variant(variant: SiteVariant) {
    if let Some(root) = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.document_element())
    {
        let _ = root.set_attribute("data-variant", variant.as_str());
    }
}

/// 读取页面根节点上的初始风格
fn document_variant() -> Option<SiteVariant> {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::window()
            .and_then(|window| window.document())
            .and_then(|document| document.document_element())
            .and_then(|root| root.get_attribute("data-variant"))
            .and_then(|value| SiteVariant::parse(&value))
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        None
    }
}
