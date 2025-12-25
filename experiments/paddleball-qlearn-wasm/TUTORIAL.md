# PaddleBall RL Tutorial (Beginner-friendly, with code)

This tutorial is written for **new Rust + WASM + WebGPU learners**. It starts from a runnable scaffold (already in this experiment) and then guides you through implementing:

- deterministic-ish fixed-timestep physics
- sparse reward and episodes
- discretized observation/state
- tabular Q-learning

If you get stuck, copy the exact code blocks in each step. Every step has a “what you should see” checkpoint.

---

## 0) Run the scaffold (verify your setup)

From repo root:

```bash
cd experiments/paddleball-qlearn-wasm/rust
wasm-pack build --target web --out-dir ../web/src/pkg
cd ../web
npm install
npm run dev
```

Open the Vite URL. You should see:
- a **teal paddle** near the bottom
- an **orange ball** moving around

If you want auto-rebuild + auto-refresh, use:

```bash
cd experiments/paddleball-qlearn-wasm/web
npm run dev:wasm:watch
```

---

## 1) Understand the current rendering pipeline (what’s already there)

### 1.1 Web entrypoint (TypeScript)

The browser boot code creates a `<canvas>` and hands it to Rust:

```1:80:experiments/paddleball-qlearn-wasm/web/src/main.ts
import init, { run } from "./pkg/paddleball_qlearn_wasm.js";

function setupCanvas(): HTMLCanvasElement {
  const app = document.getElementById("app");
  if (!app) throw new Error("Missing #app root");

  const canvas = document.createElement("canvas");
  canvas.id = "game";
  app.appendChild(canvas);

  const resize = () => {
    // Important: set actual pixel size (not just CSS size).
    const dpr = window.devicePixelRatio || 1;
    canvas.width = Math.max(1, Math.floor(window.innerWidth * dpr));
    canvas.height = Math.max(1, Math.floor(window.innerHeight * dpr));
  };

  window.addEventListener("resize", resize);
  resize();

  return canvas;
}

async function main() {
  if (!("gpu" in navigator)) {
    const msg =
      "WebGPU is not available. Try Chrome/Edge and ensure WebGPU is enabled.";
    document.body.innerHTML = `<pre style="color:#eee;padding:16px">${msg}</pre>`;
    return;
  }

  const canvas = setupCanvas();
  await init();
  // Rust owns the render loop; this just hands over the canvas.
  run(canvas);
}

main().catch((err) => {
  // eslint-disable-next-line no-console
  console.error(err);
});
```

### 1.2 Rust WASM entrypoint

Rust exports `run(canvas)` which spawns the async renderer:

```1:40:experiments/paddleball-qlearn-wasm/rust/src/lib.rs
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
```

### 1.3 Renderer overview

The renderer is in `rust/src/render.rs` and uses:
- a `SceneUniforms` uniform buffer (paddle position, ball position)
- a fullscreen triangle
- a WGSL fragment shader that draws shapes via distance functions

The shader is here:

```1:140:experiments/paddleball-qlearn-wasm/rust/src/shader.wgsl
// Fullscreen triangle + fragment-shader shapes.
// Paddle: rectangle SDF
// Ball: circle SDF
// ...
```

Checkpoint: you can change colors in `shader.wgsl` and see the page update after rebuilding.

---

## 2) Exercise 1 (beginner warm-up): move the paddle with the keyboard

Goal: teach event/state flow without RL yet.

We’ll do it in TypeScript first (simple), by passing a `paddle_x` value into Rust later.

### 2.1 Add keyboard state (TS)

Edit `web/src/main.ts` and add this **near the top**:

```ts
let heldLeft = false;
let heldRight = false;

window.addEventListener("keydown", (e) => {
  if (e.key === "ArrowLeft") heldLeft = true;
  if (e.key === "ArrowRight") heldRight = true;
});

window.addEventListener("keyup", (e) => {
  if (e.key === "ArrowLeft") heldLeft = false;
  if (e.key === "ArrowRight") heldRight = false;
});
```

Checkpoint: nothing changes yet (we’re just collecting input).

---

## 3) Create the environment “core” types (Rust) — no learning yet

We’ll implement a tiny deterministic physics loop in Rust. Start with **1D paddle** and **2D ball**.

Create a new file: `experiments/paddleball-qlearn-wasm/rust/src/world.rs`

Paste this:

```rust
#[derive(Clone, Copy, Debug)]
pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub r: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Paddle {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub max_speed: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct StepOut {
    pub reward: f32,
    pub done: bool,
    pub bounced: bool,
}

#[derive(Clone, Copy, Debug)]
pub enum Action {
    Left,
    Stay,
    Right,
}

pub struct World {
    pub ball: Ball,
    pub paddle: Paddle,
}

impl World {
    pub fn new() -> Self {
        Self {
            ball: Ball {
                x: 0.5,
                y: 0.65,
                vx: 0.30,
                vy: -0.55,
                r: 0.03,
            },
            paddle: Paddle {
                x: 0.5,
                y: 0.12,
                w: 0.20,
                h: 0.04,
                max_speed: 0.65, // units per second in UV space
            },
        }
    }

    pub fn reset_episode(&mut self) {
        *self = Self::new();
    }

    pub fn step(&mut self, action: Action, dt: f32) -> StepOut {
        // --- 1) Move paddle (discrete actions -> target velocity) ---
        let dir = match action {
            Action::Left => -1.0,
            Action::Stay => 0.0,
            Action::Right => 1.0,
        };
        let paddle_vx = dir * self.paddle.max_speed;
        self.paddle.x = (self.paddle.x + paddle_vx * dt).clamp(0.0, 1.0);

        // --- 2) Integrate ball ---
        self.ball.x += self.ball.vx * dt;
        self.ball.y += self.ball.vy * dt;

        // --- 3) Wall collisions: left/right/top reflect ---
        // left wall
        if self.ball.x - self.ball.r < 0.0 {
            self.ball.x = self.ball.r;
            self.ball.vx = self.ball.vx.abs();
        }
        // right wall
        if self.ball.x + self.ball.r > 1.0 {
            self.ball.x = 1.0 - self.ball.r;
            self.ball.vx = -self.ball.vx.abs();
        }
        // top wall
        if self.ball.y + self.ball.r > 1.0 {
            self.ball.y = 1.0 - self.ball.r;
            self.ball.vy = -self.ball.vy.abs();
        }

        // --- 4) Paddle collision (AABB overlap) ---
        let paddle_left = self.paddle.x - self.paddle.w * 0.5;
        let paddle_right = self.paddle.x + self.paddle.w * 0.5;
        let paddle_bottom = self.paddle.y - self.paddle.h * 0.5;
        let paddle_top = self.paddle.y + self.paddle.h * 0.5;

        let ball_left = self.ball.x - self.ball.r;
        let ball_right = self.ball.x + self.ball.r;
        let ball_bottom = self.ball.y - self.ball.r;
        let ball_top = self.ball.y + self.ball.r;

        let overlaps = ball_right >= paddle_left
            && ball_left <= paddle_right
            && ball_top >= paddle_bottom
            && ball_bottom <= paddle_top;

        let mut bounced = false;
        if overlaps && self.ball.vy < 0.0 {
            // Put ball above paddle and reflect upward.
            self.ball.y = paddle_top + self.ball.r;
            self.ball.vy = self.ball.vy.abs();

            // Optional: add a little horizontal "english" based on hit offset.
            let hit = ((self.ball.x - self.paddle.x) / (self.paddle.w * 0.5)).clamp(-1.0, 1.0);
            self.ball.vx += 0.25 * hit;
            bounced = true;
        }

        // --- 5) Terminal: bottom is out ---
        let done = self.ball.y + self.ball.r < 0.0;

        // --- 6) Sparse reward ---
        let reward = if bounced { 1.0 } else if done { -1.0 } else { 0.0 };

        StepOut { reward, done, bounced }
    }
}
```

Checkpoint: this compiles (after we wire it in), and the ball should bounce and episodes should end when it falls out.

---

## 4) Wire the `World` into the renderer (so rendering shows your physics)

### 4.1 Add the module

In `rust/src/lib.rs`, add:

```rust
mod world;
```

### 4.2 Store a `World` in `RenderState`

In `rust/src/render.rs`, add a field:

```rust
world: crate::world::World,
```

Initialize it in `RenderState::new(...)`:

```rust
let world = crate::world::World::new();
```

Then include `world` in `Ok(Self { ... })`.

### 4.3 Step the world in `update()`

Replace the current “sine/cos” animation in `update()` with a fixed timestep loop:

```rust
let dt = 1.0 / 60.0;
let out = self.world.step(crate::world::Action::Stay, dt);
if out.done {
    self.world.reset_episode();
}

self.uniforms.paddle_x = self.world.paddle.x;
self.uniforms.paddle_y = self.world.paddle.y;
self.uniforms.paddle_w = self.world.paddle.w;
self.uniforms.paddle_h = self.world.paddle.h;
self.uniforms.ball_x = self.world.ball.x;
self.uniforms.ball_y = self.world.ball.y;
self.uniforms.ball_r = self.world.ball.r;
```

Checkpoint: ball bounces off walls and paddle; when it falls out, it resets.

---

## 5) Add tabular Q-learning (the first real RL)

We’ll keep it very simple:
- discretize into bins
- Q-table in a `Vec<f32>`
- epsilon-greedy

Create: `rust/src/qlearn.rs` and paste:

```rust
use crate::world::{Action, World};

pub struct QLearn {
    pub epsilon: f32,
    pub alpha: f32,
    pub gamma: f32,
    pub bins_x: u32,
    pub bins_y: u32,
    pub bins_vx: u32,
    pub bins_vy: u32,
    pub bins_px: u32,
    q: Vec<f32>,
}

impl QLearn {
    pub fn new() -> Self {
        let bins_x = 12;
        let bins_y = 12;
        let bins_vx = 7;
        let bins_vy = 7;
        let bins_px = 12;
        let num_states = (bins_x * bins_y * bins_vx * bins_vy * bins_px) as usize;
        let num_actions = 3usize;

        Self {
            epsilon: 0.2,
            alpha: 0.1,
            gamma: 0.99,
            bins_x,
            bins_y,
            bins_vx,
            bins_vy,
            bins_px,
            q: vec![0.0; num_states * num_actions],
        }
    }

    pub fn num_actions(&self) -> usize {
        3
    }

    fn clamp01_to_bin(v: f32, bins: u32) -> u32 {
        let x = v.clamp(0.0, 1.0);
        let b = (x * bins as f32).floor() as u32;
        b.min(bins.saturating_sub(1))
    }

    fn clamp_to_bin(v: f32, min: f32, max: f32, bins: u32) -> u32 {
        let x = ((v - min) / (max - min)).clamp(0.0, 1.0);
        let b = (x * bins as f32).floor() as u32;
        b.min(bins.saturating_sub(1))
    }

    pub fn state_id(&self, w: &World) -> usize {
        let bx = Self::clamp01_to_bin(w.ball.x, self.bins_x);
        let by = Self::clamp01_to_bin(w.ball.y, self.bins_y);
        let bvx = Self::clamp_to_bin(w.ball.vx, -1.0, 1.0, self.bins_vx);
        let bvy = Self::clamp_to_bin(w.ball.vy, -1.0, 1.0, self.bins_vy);
        let bpx = Self::clamp01_to_bin(w.paddle.x, self.bins_px);

        // Pack: ((((bx * Y + by) * VX + bvx) * VY + bvy) * PX + bpx)
        let mut id = bx;
        id = id * self.bins_y + by;
        id = id * self.bins_vx + bvx;
        id = id * self.bins_vy + bvy;
        id = id * self.bins_px + bpx;
        id as usize
    }

    fn q_index(&self, s: usize, a: usize) -> usize {
        s * self.num_actions() + a
    }

    fn q_get(&self, s: usize, a: usize) -> f32 {
        self.q[self.q_index(s, a)]
    }

    fn q_set(&mut self, s: usize, a: usize, v: f32) {
        let idx = self.q_index(s, a);
        self.q[idx] = v;
    }

    fn greedy_action(&self, s: usize) -> usize {
        let mut best_a = 0;
        let mut best = self.q_get(s, 0);
        for a in 1..self.num_actions() {
            let v = self.q_get(s, a);
            if v > best {
                best = v;
                best_a = a;
            }
        }
        best_a
    }

    pub fn choose_action(&self, s: usize, rand01: f32) -> usize {
        if rand01 < self.epsilon {
            // random among 3
            (rand01 * 997.0).floor() as usize % self.num_actions()
        } else {
            self.greedy_action(s)
        }
    }

    pub fn update(&mut self, s: usize, a: usize, r: f32, sp: usize) {
        let qa = self.q_get(s, a);
        let mut max_next = self.q_get(sp, 0);
        for ap in 1..self.num_actions() {
            max_next = max_next.max(self.q_get(sp, ap));
        }
        let target = r + self.gamma * max_next;
        let new_q = qa + self.alpha * (target - qa);
        self.q_set(s, a, new_q);
    }

    pub fn action_from_index(a: usize) -> Action {
        match a {
            0 => Action::Left,
            1 => Action::Stay,
            _ => Action::Right,
        }
    }
}
```

Important: this uses a very dumb RNG stub (`rand01` passed in). Later you can replace it with a proper RNG (still “in-house”).

---

## 6) Training loop in the renderer (RL drives actions)

Now, in `RenderState`, store:
- `qlearn: crate::qlearn::QLearn`
- some episode stats (optional)

In `update()`:
1) compute `s`
2) pick `a`
3) step world
4) compute `s'`
5) update Q
6) if done → reset episode

Pseudo-code you can paste/translate:

```rust
let dt = 1.0 / 60.0;
let s = self.qlearn.state_id(&self.world);

// TODO: replace this with a real rand01 generator
let t = (js_sys::Date::now() as f32) * 0.001;
let rand01 = (t.fract() * 0.999).clamp(0.0, 0.999);

let a = self.qlearn.choose_action(s, rand01);
let out = self.world.step(crate::qlearn::QLearn::action_from_index(a), dt);

let sp = self.qlearn.state_id(&self.world);
self.qlearn.update(s, a, out.reward, sp);

if out.done {
    self.world.reset_episode();
}
```

Checkpoint: after some time, the paddle should start catching more often (not perfect, but trending up).

---

## 7) Make it learn faster (train faster than you render)

In `update()`, do multiple steps per frame:

```rust
for _ in 0..50 {
    // the same step/update sequence as above
}
```

Checkpoint: learning improves faster; CPU usage increases.

---

## 8) Next improvements (recommended for learners)

- Add a tiny on-screen debug overlay (episode length, reward avg).
- Implement a proper RNG (xorshift32) in Rust.
- Add epsilon decay schedule.
- Tune discretization bins (tradeoff: table size vs quality).


