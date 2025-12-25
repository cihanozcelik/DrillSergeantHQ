# Step 06 — Minimal “episode” mechanics (reset + counters)

Goal: introduce the idea of “episodes” early, with visible counters and manual reset.

## 06-01 Add an episode counter and a “steps in current episode” counter
- **What to change**: `rust/src/render.rs` (temporarily), later `world.rs`
- **Technique**: add `episode_idx: u32`, `steps_in_episode: u32` and increment each fixed sim step.
- **Why**: makes the loop observable; these become training metrics later.

## 06-02 Add a manual reset key/button (e.g., `R`) that resets ball + paddle
- **What to change**: `web/src/main.ts`, `rust/src/wasm_api.rs`
- **Technique**: add `wasm_reset_episode()` that flips a flag; Rust consumes the flag inside update.
- **Why**: avoids cross-boundary direct mutation and keeps the sim in Rust.

## 06-03 Display counters in overlay (episode #, steps)
- **What to change**: simplest is JS overlay text updated on a timer (later we’ll formalize).
- **Technique**: for now, log once per second: `episode`, `steps`.
- **Why**: visibility without UI work too early.

## 06-04 Keep the visual simulation running through resets
- **What to change**: `rust/src/render.rs`
- **Technique**: resetting just sets state back to defaults; renderer keeps running.
- **Why**: keeps architecture simple and stable.

## Code

### `rust/src/wasm_api.rs` (add reset flag)

```rust
thread_local! {
    static NEEDS_RESET: Cell<bool> = Cell::new(false);
}

#[wasm_bindgen]
pub fn wasm_reset_episode() {
    NEEDS_RESET.with(|v| v.set(true));
}

pub fn take_needs_reset() -> bool {
    NEEDS_RESET.with(|v| {
        let cur = v.get();
        if cur { v.set(false); }
        cur
    })
}
```

### `web/src/main.ts` (bind `R` key)

```ts
import init, { run, wasm_notify_resize, wasm_set_dpr, wasm_set_action, wasm_reset_episode } from "../../pkg/paddleball_qlearn_wasm.js";

window.addEventListener("keydown", (e) => {
  if (e.key === "r" || e.key === "R") {
    try { wasm_reset_episode?.(); } catch { /* ignore */ }
  }
});
```

### `rust/src/render.rs` (consume reset flag + counters)

```rust
// add fields:
episode_idx: u32,
steps_in_episode: u32,

// init:
episode_idx: 0,
steps_in_episode: 0,

// in update() inside the fixed-step loop:
if crate::wasm_api::take_needs_reset() {
    self.episode_idx += 1;
    self.steps_in_episode = 0;
    self.ball = BallState { x: 0.5, y: 0.65, vx: 0.20, vy: 0.15 };
    self.paddle_x = 0.5;
}

self.steps_in_episode += 1;
```


