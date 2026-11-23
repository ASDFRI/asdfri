//! 浏览器端水合入口。

use std::time::Duration;

use leptos::prelude::set_timeout;

/// 在 `<html>` 上标记 hydration 完成，供样式与自动化测试识别
fn mark_hydrated() {
    if let Some(root) = web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.document_element())
    {
        let _ = root.set_attribute("data-hydrated", "true");
    }
}

/// 浏览器端水合入口
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    let _ = console_log::init_with_level(log::Level::Debug);

    leptos::mount::hydrate_body(app::App);
    set_timeout(mark_hydrated, Duration::ZERO);
}
