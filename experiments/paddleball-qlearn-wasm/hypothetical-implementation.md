# 🎾 PaddleBall Q-learning (WASM + wgpu)
## Hypothetical implementation notes (pre-thinking, not the real tutorial)

This document is **not** the official step-by-step tutorial.

**Important**: this is **not deep learning** (no neural network). It’s **tabular Q-learning** (a Q-table).

What “hypothetical” means here:
- The repo currently contains a **rendering scaffold** (paddle + ball drawn by a shader).
- The sections below describe **one plausible implementation plan** (simulation + discretization + Q-learning) and how it would wire into the scaffold.
- A **real tutorial** (beginner-friendly, fully verified step-by-step) will be written later.

Use this doc as: *a design/implementation sketch you can discuss and refine before we commit to the final tutorial path.*

---

## 0) Run the current scaffold (sanity check)

From repo root:

```bash
cd experiments/paddleball-qlearn-wasm
npm install
npm run dev
```

Open the URL printed by Vite.

**Expected**: a letterboxed canvas with:
- a **teal** paddle (rectangle)
- an **orange** ball (circle)

If you don’t see those, stop here — your environment is not ready yet (WebGPU/browser setup).

---

## 1) Understand the current scaffold (no “hand-waving”)

You’re going to change this project confidently only if you can trace every frame from TypeScript → WASM → GPU.

### 1.1 The exact flow (one page mental model)

```text
web/src/main.ts
  - creates <canvas>
  - sets CSS size + backing store size (DPR-aware)
  - loads wasm-pack bundle (init())
  - calls Rust export: run(canvas)

rust/src/lib.rs
  - installs panic/log hooks
  - spawn_local(async { render::run_canvas(canvas).await })

rust/src/render.rs
  - initializes wgpu and a Surface tied to the canvas
  - builds a uniform buffer (SceneUniforms) + bind group
  - runs a requestAnimationFrame loop
  - per frame:
      update()  -> modifies SceneUniforms and uploads it (queue.write_buffer)
      render()  -> draws a fullscreen triangle (rpass.draw(0..3, 0..1))

rust/src/shader.wgsl
  - for each pixel:
      read SceneUniforms
      compute “am I inside paddle?” / “am I inside ball?” using SDF math
      output a color
```

### 1.2 Why the project uses “fullscreen triangle”

There is **no paddle mesh** and **no ball mesh**. The GPU draws exactly one triangle covering the whole screen so the fragment shader runs on every pixel. Shapes are computed from math in `fs_main`.

### 1.3 Where the CPU→GPU data boundary is

In Rust, the struct:
- `SceneUniforms` in `rust/src/render.rs`

Is uploaded every frame via:
- `queue.write_buffer(&uniforms_buffer, 0, bytemuck::bytes_of(&uniforms))`

In WGSL, that same memory is read as:
- `@group(0) @binding(0) var<uniform> scene: Scene;`

If those two ever disagree in layout/order, your visuals break.

### 1.4 Resize: why JS calls `wasm_notify_resize()`

TypeScript owns canvas sizing (`canvas.width/height`). After changing backing size, it calls:
- `wasm_notify_resize()` → sets a flag in `rust/src/wasm_api.rs`

Rust reads it in `RenderState::resize_if_needed()` and reconfigures the WebGPU surface.

If you want more detail, read `scaffold.md` (it’s a line-by-line deep dive).

---

## 2) Build the environment: deterministic-ish physics (`rust/src/world.rs`)

Right now the ball moves with sin/cos in `RenderState::update()` — that’s a placeholder. We’ll replace it with a simulation step.

### 2.1 Coordinate system (important)

The shader uses normalized UV space:
- `x` and `y` in \([0..1]\)
- (0,0) is bottom-left
- (1,1) is top-right

So your physics should also operate in **normalized coordinates**, not pixels.

### 2.2 Create `world.rs`

Create: `experiments/paddleball-qlearn-wasm/rust/src/world.rs`

Then write the structs (you can copy these *types*, but do not paste the whole step logic — you’ll implement it):

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
pub enum Action {
    Left,
    Stay,
    Right,
}

#[derive(Clone, Copy, Debug)]
pub struct StepOut {
    pub reward: f32,
    pub done: bool,
    pub bounced: bool,
}

pub struct World {
    pub ball: Ball,
    pub paddle: Paddle,
}
```

Implement:
- `World::new()`
- `World::reset_episode()`
- `World::step(action, dt) -> StepOut`

### 2.3 Implement `World::step` (the core)

You must implement these pieces in order:

- **A) Paddle movement**
  - Convert `Action` to a direction \(-1,0,+1\)
  - Apply speed: `paddle.x += dir * max_speed * dt`
  - Clamp to \([0..1]\)

- **B) Ball integration (Euler)**
  - `ball.x += ball.vx * dt`
  - `ball.y += ball.vy * dt`

- **C) Wall bounces**
  - Left/right reflect `vx`
  - Top reflect `vy`
  - Keep the ball inside bounds by snapping to edge when it penetrates

- **D) Paddle collision (AABB overlap)**
  - If overlapping and ball moving downward (`vy < 0`), reflect `vy` upward and mark `bounced = true`

- **E) Terminal + reward**
  - If ball falls below bottom: `done = true`
  - Reward:
    - `+1` on bounce
    - `-1` on terminal miss
    - `0` otherwise

Why the `vy < 0` check? It prevents “double bounces” when the ball is already traveling upward.

---

## 3) Discretization: floats → a finite `state_id` (`rust/src/discretize.rs`)

Tabular Q-learning requires a finite number of states. Our world uses floats, so we need to bin them.

Create: `experiments/paddleball-qlearn-wasm/rust/src/discretize.rs`

Implement:
- a `Bins` struct (how many buckets per variable)
- `state_id(&World, bins) -> usize`
- `num_states(bins) -> usize`

Recommended bins (good enough to learn quickly):
- ball_x: 12
- ball_y: 12
- ball_vx: 7  (map \([-1..1]\) to bins)
- ball_vy: 7
- paddle_x: 12

Key idea: **pack** multiple bin indices into one integer via base-N encoding.

---

## 4) Q-learning (`rust/src/qlearn.rs`)

Create: `experiments/paddleball-qlearn-wasm/rust/src/qlearn.rs`

Your agent stores Q-values in a flat vector:
- `q[s * num_actions + a]`

Actions:
- 0 = Left
- 1 = Stay
- 2 = Right

You must implement:
- `choose_action(s, rand01)` using epsilon-greedy
- `update(s, a, r, sp)` using the Bellman update:

\[
Q(s,a) \leftarrow Q(s,a) + \alpha \left[r + \gamma \max_{a'}Q(s',a') - Q(s,a)\right]
\]

Recommended hyperparameters:
- `epsilon = 0.2`
- `alpha = 0.1`
- `gamma = 0.99`

---

## 5) Wire it into the renderer (`rust/src/render.rs`)

Now we replace the placeholder sin/cos animation with:
- simulate → choose action → step → learn → update uniforms

### 5.1 Add modules in `rust/src/lib.rs`

Add:
- `mod world;`
- `mod discretize;`
- `mod qlearn;`

### 5.2 Add state to `RenderState`

Add fields:
- `world: crate::world::World`
- `bins: crate::discretize::Bins`
- `agent: crate::qlearn::QLearn`

Initialize them in `RenderState::new(...)` after you know the bins/state count.

### 5.3 Replace `update()` with a training loop

Conceptually:

```text
repeat N times per frame:
  s  = state_id(world)
  a  = choose_action(s)
  out = world.step(action, dt)
  sp = state_id(world)
  agent.update(s, a, out.reward, sp)
  if out.done: world.reset_episode()

then:
  uniforms = positions from world
  upload uniforms_buffer
```

Use multiple sim steps per render frame (start with `N = 50`) so learning is visible quickly.

---

## 6) What “success” looks like (and how to debug)

- **Early**: paddle moves randomly, misses often
- **Later**: paddle starts centering under the ball and catching more frequently

If it never improves:
- reduce bins (too many states makes learning slow)
- increase `N` sim steps per frame
- ensure rewards are actually happening (`bounced` flips to true sometimes)
- verify `state_id` changes with movement (log a few values)

---

## 7) You’re done (for this experiment)

At this point you have:
- a real environment (`World`)
- a discrete state encoder (`state_id`)
- a tabular RL agent (`QLearn`)
- a WebGPU renderer driven by your simulation

Next steps (optional, still PaddleBall-specific):
- add small dense shaping rewards (careful: can change behavior)
- add more state features (relative ball→paddle x, velocities)
- persist the Q-table (save/load)
