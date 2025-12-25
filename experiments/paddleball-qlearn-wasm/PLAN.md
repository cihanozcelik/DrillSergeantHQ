# Plan (Experiment #1): PaddleBall Q-learning (WASM + wgpu)

## Phase 0 (this PR): scaffold

- Browser app boots via Vite
- Rust compiles to WASM via `wasm-pack`
- `wgpu` renders a paddle + ball on a `<canvas>`
- Code is organized so that **physics + RL** can be added incrementally

## Phase 1: deterministic env

- Fixed timestep stepping (`dt = 1/60`)
- Walls: left/right/top reflect; bottom = terminal
- Paddle movement on X axis, max speed
- Paddle-ball collision yields a bounce

## Phase 2: reward + episode

- Sparse reward:
  - `+1` on paddle bounce
  - `-1` on terminal (ball out bottom)
  - `0` otherwise
- Episode reset + stats (length, total reward, catches)

## Phase 3: discretization + tabular Q-learning

- Discrete state bins for:
  - ball `(x,y)`
  - ball `(vx,vy)`
  - paddle `x`
- Q-table + epsilon-greedy policy

## Phase 4: training ergonomics

- Multiple sim steps per render frame
- Pause/step controls
- Basic on-screen stats


