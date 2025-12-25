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

## Tutorial (with code)

- Read the full beginner tutorial here: [`TUTORIAL.md`](TUTORIAL.md)

## Folder layout

```
experiments/paddleball-qlearn-wasm/
  TUTORIAL.md
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

1) Install web deps:

```bash
cd experiments/paddleball-qlearn-wasm/web
npm install
```

2) Run the dev server (this also builds Rust→WASM and watches Rust changes):

```bash
npm run dev
```

Open the URL printed by Vite.

Notes:
- You still need `wasm-pack` installed (Vite runs it via a plugin).
- If you only want to (re)build the WASM package once: `npm run build:wasm`.

## Experiment plan (phases)

- **Phase 0 (scaffold)**:
  - Vite boots a web app
  - Rust compiles to WASM via `wasm-pack`
  - `wgpu` renders a paddle + ball on a `<canvas>`
- **Phase 1 (deterministic env)**:
  - Fixed timestep (`dt = 1/60`)
  - Walls: left/right/top reflect; bottom = terminal
  - Paddle moves on X, with max speed
  - Paddle-ball collision yields a bounce
- **Phase 2 (reward + episode)**:
  - Sparse reward: `+1` bounce, `-1` terminal, `0` otherwise
  - Episode reset + stats (length, total reward, catches)
- **Phase 3 (discretization + tabular Q-learning)**:
  - Discrete bins for ball `(x,y)`, ball `(vx,vy)`, paddle `x`
  - Q-table + epsilon-greedy policy
- **Phase 4 (training ergonomics)**:
  - Multiple sim steps per render frame
  - Pause/step controls
  - Basic on-screen stats

## Experimentation knobs (what to tweak)

- **Rendering (Phase 0)**:
  - Canvas size (smaller = faster)
  - Paddle/ball colors
  - Coordinate mapping (NDC vs pixel)
- **Physics (Phase 1)**:
  - `dt`: try `1/30`, `1/60`, `1/120`
  - Ball: initial `vx/vy`, max speed clamp
  - Restitution: wall bounce coefficient, paddle bounce coefficient
  - Paddle: width/height, **max speed**
- **Reward (Phase 2)**:
  - Sparse only: `+1` bounce, `-1` terminal
  - (Later) shaped reward variants: small + for keeping ball above paddle line, small - for time
- **Discretization (Phase 3)**:
  - Bin counts: ball x/y, vx/vy, paddle x
  - Value ranges per dimension (clamp ranges)
- **Q-learning (Phase 3)**:
  - `epsilon`: start ~`0.2`, decay to `0.05`
  - `alpha`: start ~`0.1`
  - `gamma`: start ~`0.99`
  - Steps per render frame (training speed)

## What to record (recommended)

- Average episode length over last N episodes
- Average reward over last N episodes
- Catch rate
- Epsilon schedule and whether learning is stable

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


