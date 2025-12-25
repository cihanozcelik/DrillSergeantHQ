# PaddleBall Q-learning (WASM + wgpu) — Scaffold + Tutorial

This experiment is intentionally **beginner-first**: the initial code renders a **paddle** and a **ball** in the browser using **Rust + WASM + wgpu**.

You will then extend it (guided by this tutorial) into a simple RL environment:
- fixed-timestep physics (walls + paddle collision)
- sparse reward (`+1` bounce, `-1` terminal)
- discretized state
- tabular Q-learning

## What you get in the scaffold

- ✅ A Vite web app that loads a Rust WASM module.
- ✅ A Rust `cdylib` crate compiled to WASM with `wasm-pack`.
- ✅ A minimal `wgpu` render loop drawing a paddle and a ball.
- ✅ A place to implement `env.step(action)` later.

## Folder layout

```
experiments/paddleball-qlearn-wasm/
  PLAN.md
  EXPERIMENTATION.md
  rust/          # Rust -> WASM crate (wgpu rendering + future RL core)
  web/           # Vite app that loads the wasm module
```

## Prerequisites

- Rust stable
- `wasm-pack`
- Node.js (for Vite)
- A browser with WebGPU enabled (Chrome/Edge recommended)

## Running it (dev)

From the repo root:

1) Build the WASM package:

```bash
cd experiments/paddleball-qlearn-wasm/rust
wasm-pack build --target web --out-dir ../web/src/pkg
```

2) Run the web dev server:

```bash
cd ../web
npm install
npm run dev
```

Open the URL printed by Vite.

## Tutorial: how to read the code

### Step 0 — Get a picture on screen

The only goal of the scaffold is: **render a paddle and a ball**.

- The entrypoint is `web/src/main.ts`:
  - It imports the wasm package
  - It creates a `<canvas>`
  - It calls the exported Rust function `run(canvas)`
- The Rust entrypoint is `rust/src/lib.rs`:
  - It receives the canvas
  - It initializes wgpu directly from the `<canvas>`
  - Each frame, it draws two shapes

If you see a paddle and ball, you’re ready for the RL steps.

### Step 1 — Add fixed-timestep simulation (no learning yet)

Create a `World` struct in Rust with:
- `Ball { x, y, vx, vy, r }`
- `Paddle { x, w, h, max_speed }`

Implement:
- `world.step(action, dt)` which:
  - moves the paddle based on `action` with max speed
  - integrates the ball
  - handles wall collisions (left/right/top)
  - handles paddle collision
  - returns `{ reward, done }`

Keep it deterministic by using:
- constant `dt` (e.g. `1.0 / 60.0`)
- no randomness at first

### Step 2 — Add sparse reward

Return:
- `+1.0` when the ball bounces off the paddle
- `-1.0` when the ball crosses the bottom (episode ends)
- `0.0` otherwise

### Step 3 — Discretize state

Compute a small, coarse discrete representation (bins) for:
- ball position (x, y)
- ball velocity (vx, vy)
- paddle position (x)

Then pack bins into a single `state_id`.

### Step 4 — Add tabular Q-learning

Add a Q-table:
- `Vec<f32>` of length `num_states * num_actions`

Update rule:
\[
Q(s,a) \leftarrow Q(s,a) + \alpha \left[r + \gamma \max_{a'} Q(s',a') - Q(s,a)\right]
\]

Use epsilon-greedy for action selection.

### Step 5 — Train faster than you render

Render at ~60 FPS, but do multiple sim steps per frame (e.g. 10–200).
This is the easiest “fast training” trick and great for learning.

## Newbie debugging checklist

- If you see a blank page: open DevTools console; confirm the wasm module loads.
- If WebGPU is missing: enable it in your browser flags or use Chrome/Edge stable.
- If the canvas is black: ensure the render loop calls `surface.get_current_texture()` and `queue.submit(...)`.
- If it runs but is slow: reduce canvas resolution and simplify the shader.


