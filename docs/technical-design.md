# DrillSergeantHQ Technical Design

<img src="../assets/brand/drillsergeanthq-logo-512.png" alt="DrillSergeantHQ" width="180" />

This document is meant to be readable as a **standalone project file**: clear architecture, clear data flow, clear runtime boundaries—so a team can split work and execute quickly.

---

## Overview

- **Version**: 1.0  
- **Primary stack**: Rust (WASM) + `wgpu` (WebGPU) + React/TypeScript UI  
- **Core promise**: “The joy of training an agent in the browser.” The user can run a live match as a demo/sandbox, then press **Train** to start aggressive background learning; behavior visibly improves as the policy hot-swaps—**with normal controls available** (Play/Pause, speed, reset).
- **Performance stance**: multi-worker, **WASM threads** (SharedArrayBuffer + cross-origin isolation), and WebGPU compute—optimized for max throughput.

## Product overview (1 paragraph)

DrillSergeantHQ is a browser-only **agent training playground**. The user watches a live arena game (first environment: a 2D soccer-like simulation). Training starts when the user presses **Train**; before that, the simulation can run as a sandbox/demo with a dumb baseline policy (e.g., random or scripted). Once training is running, the policy is trained via massively parallel simulation (PPO), producing continuous updates. Model weights are periodically **hot-swapped** so behavior can improve in real time (either mid-session or after a reset—both are supported). The user can drag the ball/players to test instantly, or control a player and watch the opponent “get smarter” over time. In v1, user interaction does **not** enter the training dataset.

## Browser constraints that lock the design

- **WebGPU works in Dedicated Workers** via `WorkerNavigator.gpu`. ([MDN Web Docs][1])
- **Rendering can move off the main thread** using `OffscreenCanvas` + `transferControlToOffscreen()`. ([MDN Web Docs][2])
- **SharedArrayBuffer + WASM threads require cross-origin isolation** (COOP/COEP). ([web.dev][3])
- **`wasm-bindgen-rayon` provides Rayon-style parallelism on WASM** using Web Workers + SharedArrayBuffer. ([docs.rs][4])
- **`DedicatedWorkerGlobalScope.requestAnimationFrame` exists in workers** (clean render loop option). ([MDN Web Docs][5])

## Goals and non-goals

### Goals

- **Browser-only**: no native app requirement.
- **Max performance**: saturate CPU cores; use GPU for both rendering and training compute.
- **Continuous improvement UX (optional)**: allow the show match to keep running while policy weights hot-swap, so behavior can improve without requiring a reset.
- **Normal controls**: Play/Pause, speed control, and reset must be supported.
- **Training isolation (v1)**: show env and user input do not contaminate training data.
- **Single-page experience**: instant “training is happening” feeling on load.

### Non-goals (v1)

- Shipping with a framework-dependent RL stack (e.g., TF.js). Core training stays in our control.
- Cloud/server-side compute (runs entirely on-device).
- Multi-tab and multi-user synchronization.

## Dependency policy (what we allow vs. what we build)

This project is intentionally **from-scratch and highly optimized for this specific use case**. We only allow dependencies that are effectively *platform/protocol glue*; we do **not** import whole engines or training stacks.

### Allowed (platform / protocol “glue”)

- **WebGPU access / abstraction**: `wgpu` (or equivalent thin WebGPU bindings)
- **WASM ↔ JS interop**: `wasm-bindgen`, `js-sys`, `web-sys`
- **Tooling / build glue**: minimal build dependencies required to compile and bundle WASM/TS

### Built in-house (core logic)

- **Simulation**: physics, collisions, deterministic stepping, vectorized SoA batches (`sim_core`)
- **RL**: PPO/GAE math, rollout/advantage computation, self-play pool logic (`rl_core`)
- **Model**: tiny MLP inference for rollouts + training model definition (`nn_cpu`, trainer-side model)
- **Training system**: rollout orchestration, schedulers, checkpoint publishing (`trainer`)
- **GPU kernels**: WGSL sources + binding layouts, training compute kernels (`gpu_kernels`)
- **Runtime/boot**: WASM exports, worker bootstrapping, SAB setup/ABI (`wasm_entry`, `/web` runtime)

### Disallowed (too “high-level” / defeats the purpose)

- **Full game engines / physics engines**: e.g. Bevy, Rapier, Box2D ports
- **Full ML/RL frameworks** (Rust or JS): e.g. Candle, Burn, tch-rs/PyTorch bindings, TF.js training stacks
- **“Black box” PPO implementations** where we can’t fully control memory layout, batching, and compute scheduling

### Rule of thumb

- If the dependency **implements the product’s core value** (sim/RL/NN/training), it’s **not allowed**.
- If it **only exposes browser/OS protocols** (WebGPU, WASM interop), it’s **allowed**.

## System architecture (worker topology + CPU/GPU plan)

### Components

#### 1) Main thread (React UI)

- DOM/UI, control panels, settings
- Input capture (mouse/keyboard/touch) + minimal event packaging
- Creates `HTMLCanvasElement`; transfers an `OffscreenCanvas` to the Render/Eval worker
- Optionally pulls training metrics at 1–2 Hz

#### 2) Render/Eval Worker (show environment + real-time rendering)

- Hosts the single **show env** (the match the user watches)
- Fixed-timestep simulation (e.g., 120 Hz) + rendering (60 FPS)
- Policy inference drives AI players (e.g., 10–30 Hz action selection)
- WebGPU render pipeline (`wgpu`)
- Hot-swaps “weights checkpoints” from the Trainer (e.g., every 1s)

#### 3) Trainer Worker (learner + orchestrator)

- PPO/GAE update loop
- WebGPU compute for large-batch forward/backward + Adam updates
- Manages rollout sources:
  - **Option A**: a single worker uses `wasm-bindgen-rayon` thread pool to parallelize env batches
  - **Option B (max scale)**: a Rollout Worker Pool (N workers) streams experiences via SAB ring buffer

#### 4) Rollout Worker Pool (N workers, CPU)

- Headless simulation + inference (mostly CPU)
- Thousands of env instances per worker; continuously generates rollouts
- Writes experiences into a SharedArrayBuffer ring buffer (copy-free)

### Macro diagram

```text
+-------------------------+    one-time canvas transfer    +------------------------------+
| Main Thread (React UI)  | -----------------------------> | Render/Eval Worker           |
| - UI, input capture     |                                | - Show env (1)               |
| - minimal telemetry UI  |                                | - Sim 120Hz + Render 60fps   |
+-----------+-------------+                                | - Inference + hot-swap       |
            |                                             +--------------+---------------+
            | small control msgs                                           ^
            v                                                               |
+------------------------------+     SAB weights double-buffer              |
| Trainer Worker               | -------------------------------------------+
| - PPO learner (WebGPU)       |
| - checkpoint publisher       |     SAB rollout ring buffer
+--------------+---------------+ <----------------------------------+
               ^                                                   |
               |                                                   |
               |                                                   v
+-----------------------------+                  +------------------------------+
| Rollout Worker Pool (N)     |                  | (optional) metrics           |
| - headless env batches      |                  | to UI 1-2Hz                  |
| - writes rollouts to SAB    |                  +------------------------------+
+-----------------------------+
```

## Shared data strategy: fewer messages, more SharedArrayBuffer

### Weights sharing (Trainer → Render/Eval)

Recommended: **SharedArrayBuffer double buffer + atomic versioning**.

Memory:

- `weights_A`: SAB (`Float32Array`)
- `weights_B`: SAB (`Float32Array`)
- `weights_ctrl`: SAB (`Int32Array`)

`weights_ctrl` layout (Int32 indices):

- `[0] active_idx` (0/1)
- `[1] version` (monotonically increasing)
- `[2] shape_hash` (optional; detects network architecture changes)
- `[3] step_counter` (optional; useful for UI)

Trainer publish:

- Compute `inactive = 1 - active_idx`
- Write all weights into the inactive buffer (single contiguous write)
- `Atomics.store(active_idx, inactive)`
- `Atomics.add(version, 1)`

Render hot-swap:

- Every `swap_interval_ms` (e.g., 1000ms):
  - `v = Atomics.load(version)`
  - if `v != last_v`:
    - `idx = Atomics.load(active_idx)`
    - switch the policy’s weight pointer to `idx`
    - `last_v = v`

**No simulation jump**: state remains the same; only the decision function changes.

### Rollout ring buffer (Rollout workers → Trainer)

Goal: **N producers** (rollout workers) → **1 consumer** (trainer) with copy-free throughput.

Memory:

- `rollout_data`: SAB (mixed float/int segments as needed)
- `rollout_ctrl`: SAB (atomics)

Control fields (example):

- `head` (write index)
- `tail` (read index)
- `capacity`
- `dropped` counter (optional)

Rollout record layout: **SoA recommended**, e.g.:

- `obs[t][env]`, `act[t][env]`, `rew[t][env]`, `done[t][env]`, `logp[t][env]`, `value[t][env]`

## Timing and loops (real-time show + background training)

### Render/Eval worker loop

- **Render**: 60 FPS
- **Simulation**: fixed 120 Hz (accumulator)
- **Action selection**: 10–30 Hz (action repeat)
- **Weights swap**: 1 Hz (every second) or “on version change”

Suggested parameters:

- `sim_dt = 1/120`
- `action_repeat = 4` → 30 Hz policy decisions
- Use `requestAnimationFrame` in the worker where available; otherwise fallback to `setInterval`

### Training loop

- **Rollout**: continuous (keep CPU cores busy)
- **Update**: PPO (GPU compute), large batches
- **Publish**: after each update or at 1 Hz

## Simulation design (2D arena soccer — deterministic + vectorized)

### World

- Rectangular field with goal recesses (static segment/polygon boundaries)
- Entities:
  - **Ball**: circle collider
  - **Player0/1**: rotatable square (OBB)

### Physics

- Semi-implicit Euler
- Collisions:
  - Ball vs walls: reflection + restitution
  - Ball vs OBB: closest-point + impulse
  - OBB vs walls: projection/constraint
  - (Optional) player vs player: OBB–OBB push or simplified resolution

### Control (no “teleport jumps”)

- Action = target velocity/acceleration + target angular velocity (clamped)
- Dash = cooldown + bounded impulse (no teleport)
- This ensures policy changes change behavior without invalidating physics state continuity.

### EnvBatch layout (SoA, for training)

- `ball_px[N]`, `ball_py[N]`, `ball_vx[N]`, `ball_vy[N]`
- `p0_px[N]`, `p0_py[N]`, `p0_ang[N]`, `p0_vx[N]`, `p0_vy[N]`, `p0_w[N]`
- `p1_...`
- `done[N]`, `score0[N]`, `score1[N]`, `step[N]`
- `rng_state[N]`

Show env (Render/Eval) is a single env instance using the same simulation code.

## RL algorithm (stable + fast): PPO + GAE + self-play pool

### Why PPO?

- Simple on-policy pipeline: rollout → update
- Stable due to clipped surrogate objective. ([arXiv][6])

### Model

- Small MLP (2–3 layers), e.g. 64/128 hidden units
- Outputs:
  - Policy logits (discrete actions)
  - Value \(V(s)\)

### Action space (discrete, fast to learn)

Example with 30 actions:

- Move: `{stop, fwd, back, strafeL, strafeR}` (5)
- Turn: `{none, left, right}` (3)
- Dash: `{0, 1}` (2)

Total: \(5 \times 3 \times 2 = 30\)

### Reward

Terminal:

- Goal for: +1
- Goal against: -1
- Timeout: 0 or small negative

Shaping (learnability):

- Potential-based shaping: \(\Phi(s) = -dist(ball, opp\_goal) + dist(ball, own\_goal)\)
- \(r \mathrel{+}= \gamma \Phi(s') - \Phi(s)\)

### GAE

\[
\delta_t = r_t + \gamma (1 - done_t) V(s_{t+1}) - V(s_t)
\]
\[
A_t = \delta_t + \gamma \lambda (1 - done_t) A_{t+1}
\]
\[
R_t = A_t + V(s_t)
\]

### PPO loss

- \(ratio = \exp(\log p_{new} - \log p_{old})\)
- \(L_{clip} = -\mathbb{E}[\min(ratio \cdot A, \mathrm{clip}(ratio, 1-\epsilon, 1+\epsilon)\cdot A)]\)
- \(L_v = c_v \cdot \mathbb{E}[(V - R)^2]\)
- \(L_{ent} = -c_e \cdot \mathbb{E}[H(\pi)]\)
- \(L = L_{clip} + L_v + L_{ent}\)

### Self-play pool

- In training envs, both sides are AI
- Stabilization: opponent pool (sample from mixed snapshots)
- Show env modes:
  - AI vs AI (latest vs pool)
  - Human vs latest

## GPU strategy (render + learner compute + optional GPU simulation)

### Where WebGPU is used

- Render/Eval worker: WebGPU rendering (`wgpu`)
- Trainer worker: WebGPU compute (PPO forward/backward/update)

References: `WorkerNavigator.gpu` ([MDN Web Docs][1]), WebGPU spec ([W3C][7]).

### Training compute kernels (minimum set)

1. `linear_forward`: \(Y = XW + b\)
2. `activation_forward`: ReLU/Tanh
3. Policy/value heads
4. Softmax + logprob + entropy
5. Advantage normalization (reduce)
6. PPO loss reduction
7. `backward_linear` + `backward_activation`
8. `adam_update`

### Optional (planned for max-perf): GPU simulation backend

Goal: move environment stepping to GPU compute as well.

- `state_buffer` (SoA) on GPU
- `action_buffer` on GPU
- Kernel `step_envs` (1 thread = 1 env)

Main challenge: OBB collision/impulse debugging cost in WGSL.

Strategy:

- Design supports GPU sim as a first-class backend (SoA layout, fixed-size kernels).
- Implementation order: stabilize CPU rollouts + GPU learner first, then add GPU rollout backend.

## Build and deploy (COOP/COEP, SAB, WASM threads)

### Cross-origin isolation is required

SharedArrayBuffer requires cross-origin isolation. ([web.dev][3])

Required headers (production):

- `Cross-Origin-Opener-Policy: same-origin`
- `Cross-Origin-Embedder-Policy: require-corp`

MDN reference for COOP/COEP: ([MDN Web Docs][8])

### WASM threads + Rayon

- Use `wasm-bindgen-rayon`
- Initialize thread pool during Trainer Worker boot

References: `wasm-bindgen-rayon` docs ([docs.rs][4])

### OffscreenCanvas worker rendering

- Transfer canvas control to a worker:
  - `HTMLCanvasElement.transferControlToOffscreen()` ([MDN Web Docs][9])
  - OffscreenCanvas overview ([MDN Web Docs][2])

## Code organization (monorepo)

### `/web` (React + TypeScript)

- `src/ui` (panels, modes, settings)
- `src/workers` (`render.worker.ts`, `trainer.worker.ts`, `rollout.worker.ts`)
- `src/runtime` (SAB init, message codecs)

### `/rust` (workspace)

- `sim_core` (2D physics + env step, SoA)
- `rl_core` (obs/action encoding, reward, GAE, PPO math)
- `nn_cpu` (tiny MLP inference for rollout)
- `trainer` (orchestrator logic; binds WebGPU compute)
- `gpu_kernels` (WGSL sources + binding layouts)
- `wasm_entry` (wasm-bindgen exports, init, hooks)

## Protocols (strict contracts)

### UI → Render/Eval worker

- `INIT_CANVAS { offscreen, w, h, dpi }`
- `SET_MODE { ai_vs_ai | human_vs_ai | sandbox }`
- `INPUT_BATCH { events[] }` (pointer/key)
- `SHOW_CONFIG { action_repeat, swap_interval_ms, blend_ms }`
- `PLAY`
- `PAUSE`
- `RESET_SHOW { seed? }`
- `SET_SIM_SPEED { multiplier }` (e.g., 0.25x, 1x, 2x, 4x)

### UI → Trainer worker

- `START_TRAINING { seed, num_envs, T, gamma, lambda, clip, lr, epochs, minibatch, ... }`
- `STOP_TRAINING`
- `SET_HYPER { key, value }`
- `RESET_TRAINER`

### Trainer → Render/Eval (SAB weights)

- `weights_A/B` SAB + control SAB (`active_idx`, `version`)

### Rollout → Trainer (SAB ring)

- `rollout_data` SAB + control SAB (`head`, `tail`)

## Implementation plan (milestones)

### Phase A — Foundation (COOP/COEP + workers + WASM threads)

- Vite/React template + COOP/COEP dev server config
- WASM build pipeline
- Trainer Worker: WASM module + Rayon thread pool init
- Render Worker: take `OffscreenCanvas` + WebGPU device init (`wgpu`)
- SAB control blocks

Definition of Done:

- `crossOriginIsolated === true`
- Rendering runs in a worker
- Rayon parallelism runs in a worker

### Phase B — CPU sim + show rendering

- 2D physics + collisions
- Deterministic fixed timestep
- Show env sim + render in Render/Eval worker (60 FPS)
- UI input (drag/human control)
- Baseline AI (random)

### Phase C — Rollout engine (N envs) + RL data structures

- SoA env batch
- Random scenario generator
- Rollout buffer (obs, action, reward, done, value, logp)
- GAE + returns

### Phase D — PPO update (GPU compute)

- WebGPU compute foundation (Trainer Worker)
- MLP forward kernels
- PPO loss + backward + Adam kernels
- Minibatch/epoch schedule
- Checkpoint publish (SAB double buffer)

### Phase E — Show hot-swap + “live improvement” UX

- Render worker policy hot-swap (1 Hz or on version change)
- Optional blending (\(\pi_{old}/\pi_{new}\) mix)
- Demo target: visible improvement within 30–120 seconds

### Phase F — Self-play pool + stabilization

- Snapshot pool
- Opponent sampling
- Win-rate EMA

### Phase G — GPU sim backend (optional, for max-perf)

- WGSL `step_envs` kernel
- Behavioral verification against CPU sim
- Hybrid scheduler (CPU rollout + GPU rollout)

## Testing and validation

### Simulation tests

- Determinism (same seed → same results)
- Collision invariants (energy should not increase)
- Goal detection edge cases

### RL tests

- Sanity env: “approach the ball” should increase reward over time
- Finite-difference gradient check (small model/batch)

### Performance tests

- Rollout steps/s
- Updates/s
- GPU time (timestamp queries optional)
- Show FPS

## Risks and mitigations

- COOP/COEP deployment friction → provide explicit hosting recipes (nginx/vercel/cloudflare). ([web.dev][3])
- WASM threads setup complexity → follow `wasm-bindgen-rayon` recommended approach. ([docs.rs][4])
- GPU kernel debugging pain → keep a CPU reference path (debug mode) for cross-checking.
- Hot-swap “snap” behavior → optional blending; simulation state never changes.

## Default “max-perf” configuration (auto-tuned)

- `num_rollout_workers`: based on CPU cores (e.g., `cores - 2`)
- `envs_per_worker`: 1024–8192 (device dependent)
- `rollout_T`: 128–512
- `action_repeat`: 4–6
- PPO: epochs 2–4, minibatch 2048–8192
- `swap_interval_ms`: 1000 (1 Hz) (can go down to ~250ms if desired)

## Related demos and inspiration (kept as references)

- **TensorFlow.js CartPole** (working RL training demo in the browser)
  - tfjs-examples CartPole README: ([GitHub][10])
  - Live demo page: ([Google Cloud Storage][11])
  - TF.js demos list: ([TensorFlow][12])
- **Flappy Bird RL** (browser demo / model plays)
  - `wangjia184/rl` repo: ([GitHub][13])
  - Background reading example: ([Medium][14])
- **TensorFlow Playground** (browser NN training visualization)
  - Playground app: ([TensorFlow Playground][15])
- **GAN Lab** (browser training + visualization; strong UI inspiration)
  - GAN Lab site: ([GT Data Science Polo Club][16])
  - GAN Lab paper (VAST 2018): ([arXiv][17])
- **Sequence Toy** (WebGPU LM training playground)
  - Sequence Toy homepage: ([Sequence Toys][18])
  - “Train a language model in your browser” blog post: ([Vin Howe][19])
  - HN thread: ([Hacker News][20])

## Product naming (DrillSergeantHQ)

- **Platform**: DrillSergeantHQ
- **Environment / level naming system** (extensible):
  - Ball Drill (first env: 2D soccer/arena)
  - Maze Drill
  - Sumo Drill
  - Chase Drill
  - etc.

This avoids being “locked to soccer” and keeps the “train anything” feel.

## Optional next add-ons (to speed up implementation)

If you want, the next step can be two additional specs (without changing this doc’s intent):

1. **Definitive schema** for OBS vector / ACTION encoding / reward formulas (dimensions, normalization, tables)
2. **SAB rollout ring buffer byte layout** (offset/stride) + a TS/Rust shared ABI contract

[1]: https://developer.mozilla.org/en-US/docs/Web/API/WorkerNavigator/gpu?utm_source=chatgpt.com "WorkerNavigator: gpu property - Web APIs | MDN"
[2]: https://developer.mozilla.org/en-US/docs/Web/API/OffscreenCanvas?utm_source=chatgpt.com "OffscreenCanvas - Web APIs | MDN"
[3]: https://web.dev/articles/cross-origin-isolation-guide?utm_source=chatgpt.com "A guide to enable cross-origin isolation | Articles"
[4]: https://docs.rs/wasm-bindgen-rayon?utm_source=chatgpt.com "wasm_bindgen_rayon - Rust"
[5]: https://developer.mozilla.org/en-US/docs/Web/API/DedicatedWorkerGlobalScope/requestAnimationFrame?utm_source=chatgpt.com "requestAnimationFrame() method - Web APIs - MDN Web Docs"
[6]: https://arxiv.org/abs/1707.06347?utm_source=chatgpt.com "Proximal Policy Optimization Algorithms"
[7]: https://www.w3.org/TR/webgpu/?utm_source=chatgpt.com "WebGPU"
[8]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Cross-Origin-Embedder-Policy?utm_source=chatgpt.com "Cross-Origin-Embedder-Policy (COEP) header - HTTP | MDN"
[9]: https://developer.mozilla.org/en-US/docs/Web/API/HTMLCanvasElement/transferControlToOffscreen?utm_source=chatgpt.com "HTMLCanvasElement: transferControlToOffscreen() method"
[10]: https://github.com/tensorflow/tfjs-examples/blob/master/cart-pole/README.md?utm_source=chatgpt.com "tfjs-examples/cart-pole/README.md at master · tensorflow ..."
[11]: https://storage.googleapis.com/tfjs-examples/cart-pole/dist/index.html?utm_source=chatgpt.com "TensorFlow.js: Reinforcement Learning - Googleapis.com"
[12]: https://www.tensorflow.org/js/demos?utm_source=chatgpt.com "TensorFlow.js demos"
[13]: https://github.com/wangjia184/rl?utm_source=chatgpt.com "wangjia184/rl: Train an AI agent to play FlappyBird"
[14]: https://medium.com/%40ks2496/teaching-a-bird-to-fly-training-flappy-bird-ai-in-the-browser-with-tensorflow-d8aa90543e2c?utm_source=chatgpt.com "Training Flappy Bird AI in the Browser with TensorFlow"
[15]: https://playground.tensorflow.org/?utm_source=chatgpt.com "A Neural Network Playground - TensorFlow"
[16]: https://poloclub.github.io/ganlab/?utm_source=chatgpt.com "GAN Lab: Play with Generative Adversarial Networks in ..."
[17]: https://arxiv.org/abs/1809.01587?utm_source=chatgpt.com "GAN Lab: Understanding Complex Deep Generative Models using Interactive Visual Experimentation"
[18]: https://sequence.toys/?utm_source=chatgpt.com "sequence toy"
[19]: https://vin.how/blog/train-a-language-model-in-your-browser?utm_source=chatgpt.com "You probably shouldn't train a language model in ... - Vin Howe"
[20]: https://news.ycombinator.com/item?id=46004356&utm_source=chatgpt.com "Train a language model in the browser with WebGPU"

