# Step 03 — Real time delta + safety clamps

Goal: make simulation stable under real-world browser behavior (tab-switch, slow frames).

## 03-01 Measure frame-to-frame time (`dt_seconds`) using a real clock source
- **What to change**: `rust/src/render.rs`
- **Technique**: use `performance.now()` and compute:
  - `dt_seconds = (now_ms - last_ms) / 1000.0`
- **Why**: browser frames are not constant; you must measure time.

## 03-02 Clamp dt so tab-switch doesn’t cause giant jumps
- **What to change**: `rust/src/render.rs`
- **Technique**: clamp to a max value (50–100ms) before integrating.
- **Why**: prevents tunneling through walls and huge jumps in one update.

## 03-03 Show debug values: `dt_ms` and “steps/sec”
- **What to change**: `rust/src/render.rs` (or later a JS overlay)
- **Technique**:
  - log dt_ms once per second
  - optionally log “frames/sec” (count frames over ~1s)
- **Why**: makes performance and timing problems visible.

## 03-04 Verify paddle movement feels consistent
- **What to do**: manual testing
- **Technique**:
  - resize window
  - background the tab for a few seconds and return
  - confirm no giant leaps
- **Why**: time bugs will destroy learning later.

## Code

If you followed Step 01, you already have `dt` and a clamp. Here’s the key pattern:

```rust
let now_ms = window.performance().unwrap().now();
let mut dt = ((now_ms - self.last_frame_ms) * 0.001) as f32;
self.last_frame_ms = now_ms;
dt = dt.clamp(0.0, 0.05);
```


