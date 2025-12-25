use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use web_sys::HtmlCanvasElement;

mod render;

/// Web entrypoint: called from `web/src/main.ts`.
///
/// For the scaffold, we only render a paddle + ball. The RL/env pieces are
/// intentionally left as TODOs for the tutorial.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn run(canvas: HtmlCanvasElement) {
    // Better panic messages in the browser console.
    console_error_panic_hook::set_once();
    let _ = console_log::init_with_level(log::Level::Info);

    wasm_bindgen_futures::spawn_local(async move {
        if let Err(err) = render::run_canvas(canvas).await {
            log::error!("fatal: {err:?}");
        }
    });
}

/// Native entrypoint placeholder.
///
/// This experiment is primarily browser-first, but keeping a native stub makes
/// it easy to add `cargo run` later if you want.
#[cfg(not(target_arch = "wasm32"))]
pub fn run_native_placeholder() {
    eprintln!("This experiment is intended to run in the browser (wasm32).");
}


