# Step 05 — Keyboard input (user can move the paddle)

Goal: connect browser keyboard input → WASM → Rust simulation.

## 05-01 Add keyboard listeners in `web/src/main.ts`
- **What to change**: `web/src/main.ts`
- **Technique**: listen to `keydown`/`keyup` for ArrowLeft/ArrowRight and compute a direction \(-1, 0, +1\).
- **Why**: simplest input method; no UI dependencies.

## 05-02 Add a WASM export to set current action (like the resize flag pattern)
- **What to change**: `rust/src/wasm_api.rs`
- **Technique**: add a `thread_local Cell<i32>` and a `#[wasm_bindgen]` setter.
- **Why**: avoids borrowing/lifetime issues across the JS↔WASM boundary; matches existing code style.

## 05-03 Read the action in Rust and apply it to paddle motion
- **What to change**: `rust/src/render.rs`
- **Technique**: call `crate::wasm_api::get_action_dir()` each update and integrate:
  - `paddle_x += dir * max_speed * dt_fixed`
- **Why**: simulation stays in Rust; JS only sends input.

## 05-04 Add a tiny “controls help” overlay (keys)
- **What to change**: `web/index.html` or DOM in `web/src/main.ts`
- **Technique**: add simple text overlay: “←/→ move, R reset (later)”.
- **Why**: prevents “what do I press?” frustration for newbies.

## Code

### `rust/src/wasm_api.rs` (add action storage + API)

```rust
use std::cell::Cell;
use wasm_bindgen::prelude::*;

thread_local! {
    static CURRENT_ACTION: Cell<i32> = Cell::new(0); // -1 left, 0 stay, +1 right
}

#[wasm_bindgen]
pub fn wasm_set_action(a: i32) {
    CURRENT_ACTION.with(|v| v.set(a.clamp(-1, 1)));
}

pub fn get_action_dir() -> f32 {
    CURRENT_ACTION.with(|v| v.get() as f32)
}
```

### `web/src/main.ts` (send action to WASM)

```ts
import init, { run, wasm_notify_resize, wasm_set_dpr, wasm_set_action } from "../../pkg/paddleball_qlearn_wasm.js";

let leftDown = false;
let rightDown = false;

function updateAction() {
  const dir = (rightDown ? 1 : 0) + (leftDown ? -1 : 0);
  try { wasm_set_action?.(dir); } catch { /* ignore */ }
}

window.addEventListener("keydown", (e) => {
  if (e.key === "ArrowLeft") { leftDown = true; updateAction(); }
  if (e.key === "ArrowRight") { rightDown = true; updateAction(); }
});

window.addEventListener("keyup", (e) => {
  if (e.key === "ArrowLeft") { leftDown = false; updateAction(); }
  if (e.key === "ArrowRight") { rightDown = false; updateAction(); }
});
```

### `rust/src/render.rs` (use the action dir)

```rust
let dir = crate::wasm_api::get_action_dir();
self.paddle_x += dir * paddle_max_speed * self.dt_fixed;
```


