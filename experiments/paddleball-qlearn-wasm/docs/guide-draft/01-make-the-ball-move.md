# Step 01 — Make the ball move (first visible progress)

Goal: get **visible motion** with a tiny Rust-side state and drive shader uniforms from it.

## 01-01 Add a tiny “world state” (ball position + velocity) stored in Rust
- **What to change**: `rust/src/render.rs`
- **Technique**: add a small `BallState` struct and store it on `RenderState`.
- **Why**: fastest path to “something moves” without refactoring too early.

## 01-02 Update that state every animation frame using a simple per-frame delta
- **What to change**: `rust/src/render.rs`
- **Technique**: use `performance.now()` to compute `dt_seconds`, then Euler integrate:
  - `x += vx * dt`, `y += vy * dt`
- **Why**: Euler is simple and good enough for this learning experiment.

## 01-03 Drive `SceneUniforms.ball_x/ball_y` from that state (no hardcoded positions)
- **What to change**: `rust/src/render.rs`
- **Technique**: replace the scaffold’s static assignments with:
  - `uniforms.ball_x = ball.x`
  - `uniforms.ball_y = ball.y`
- **Why**: the shader should be “dumb”; the CPU owns simulation.

## 01-04 Add one debug number (console or overlay): current ball `(x, y)`
- **What to change**: `rust/src/render.rs`
- **Technique**: log once per second (overlay comes later).
- **Why**: provides fast feedback that dt + state are updating.

## Code (covers 01-01..01-04)

Add code like this to `rust/src/render.rs`:

```rust
// near the top, after SceneUniforms
#[derive(Clone, Copy, Debug)]
struct BallState {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

#[derive(Clone, Copy, Debug)]
struct DebugTimers {
    last_log_ms: f64,
}

// in RenderState struct add:
ball: BallState,
dbg: DebugTimers,
last_frame_ms: f64,

// in RenderState::new(...) after uniforms init:
let now_ms = web_sys::window()
    .and_then(|w| w.performance())
    .map(|p| p.now())
    .unwrap_or(0.0);

let ball = BallState { x: 0.5, y: 0.65, vx: 0.20, vy: 0.15 };
let dbg = DebugTimers { last_log_ms: now_ms };

// include in Ok(Self { ... }):
ball,
dbg,
last_frame_ms: now_ms,

// in fn update(&mut self) replace static scene with:
let window = web_sys::window().unwrap();
let now_ms = window.performance().unwrap().now();
let mut dt = ((now_ms - self.last_frame_ms) * 0.001) as f32;
self.last_frame_ms = now_ms;

// clamp dt to avoid giant jumps (formalized in Step 03)
dt = dt.clamp(0.0, 0.05);

// integrate ball
self.ball.x += self.ball.vx * dt;
self.ball.y += self.ball.vy * dt;

// drive uniforms
self.uniforms.ball_x = self.ball.x;
self.uniforms.ball_y = self.ball.y;

// debug log ~1 Hz
if now_ms - self.dbg.last_log_ms > 1000.0 {
    self.dbg.last_log_ms = now_ms;
    log::info!("ball x={:.3} y={:.3} dt_ms={:.1}", self.ball.x, self.ball.y, dt * 1000.0);
}

// keep the upload:
self.queue
    .write_buffer(&self.uniforms_buffer, 0, bytemuck::bytes_of(&self.uniforms));
```


