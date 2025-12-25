use std::cell::Cell;

use wasm_bindgen::prelude::*;

thread_local! {
    static NEEDS_RESIZE: Cell<bool> = Cell::new(false);
    static DEVICE_PIXEL_RATIO: Cell<f32> = Cell::new(1.0);
}

/// Called by JS after it changes the canvas backing size.
#[wasm_bindgen]
pub fn wasm_notify_resize() {
    NEEDS_RESIZE.with(|v| v.set(true));
}

/// Let JS tell WASM what DPR it's using for canvas backing size.
#[wasm_bindgen]
pub fn wasm_set_dpr(dpr: f32) {
    let dpr = if dpr.is_finite() && dpr > 0.0 { dpr } else { 1.0 };
    DEVICE_PIXEL_RATIO.with(|v| v.set(dpr));
}

pub fn take_needs_resize() -> bool {
    NEEDS_RESIZE.with(|v| {
        let cur = v.get();
        if cur {
            v.set(false);
        }
        cur
    })
}

#[allow(dead_code)]
pub fn get_dpr() -> f32 {
    DEVICE_PIXEL_RATIO.with(|v| v.get())
}


