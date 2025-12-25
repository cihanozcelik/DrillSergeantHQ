# PaddleBall Q-learning (WASM + wgpu) — Development Plan (Newbie-Friendly)

This file is the **official, repo-local plan** for the learning experiment in:
`experiments/paddleball-qlearn-wasm/`.

It complements:
- `README.md` (how to run)
- `scaffold.md` (how the current render scaffold works)
- `guide-draft/README.md` (step-by-step implementation guide (draft; not yet proven))

---

## Final form: how a user engages with the experiment

In the finished “usable learning experiment”, a beginner should be able to:

1) **Open the page** and immediately see:
- A canvas (paddle + ball)
- A small control panel / overlay with a few obvious actions and metrics

2) **Use three modes**:
- **Play (Manual)**: the user controls the paddle (keyboard and/or on-screen buttons).
- **Train**: the agent learns (Q-learning updates ON).
- **Evaluate**: the agent plays (learning updates OFF, typically epsilon = 0), so improvement is measurable.

Notes for clarity:
- **“Stop training / Pause”** means: keep the simulation running, but **stop updating** the Q-table. This lets you immediately see “what it has learned so far”.
- **The paddle should not teleport**: even when actions are random early on, paddle motion is still constrained by a **max speed** and clamped bounds.

3) **Press a few core controls** (minimum UX to be “usable”):
- **Train / Stop training**
- **Reset episode**
- **Reset training** (clears Q-table)
- **Speed** control (sim steps per render frame, or steps/sec)
- **Exploration (epsilon)** control
- **Manual interference** toggle:
  - While training, allow the user to temporarily override the agent action (or blend actions) to see cause/effect.
- **Save / Load Q-table** (optional but strongly recommended so progress persists)

4) **Understand progress** through beginner-friendly metrics:
- Episode length (steps)
- Bounces per episode
- Moving average of bounces/episode
- Steps/sec
- Current mode (Manual/Train/Eval) and current epsilon

Training speed expectation:
- **Play / Evaluate** typically run “real-time-ish” (e.g., ~1 fixed-timestep step per render frame).
- **Train** often runs “fast-forward” (e.g., K=50–500 sim steps per render frame) so learning progress is visible quickly.

---

## Milestone definition: “usable” for this experiment

This experiment is considered **usable** when a beginner can:
- Play manually and understand the goal.
- Start training and observe improvement within a couple minutes.
- Stop training and evaluate the learned policy.
- Reset training and try again with different parameters.

---

## Development steps (22 steps; each step has 3+ testable todos)

> Assumption: the **rendering scaffold already works** (see `scaffold.md`). This plan starts **after** the scaffold and focuses on building the RL learning experiment on top.

### Step 01 — Make it move (visible progress: ball + paddle)
- [ ] Implement the smallest possible `World` that supports: ball position/velocity + paddle position.
- [ ] In Rust, advance the world once per frame with a simple `dt` (even if it’s rough at first).
- [ ] Render from world state (ball/paddle positions come from the world, not hardcoded uniforms).
- **Done when**: You can see the ball drifting in a direction and the paddle sliding left/right (no collisions required yet).

### Step 02 — Time step basics (make motion consistent)
- [ ] Switch from “dt per frame guessed” to a real time source (frame-to-frame delta).
- [ ] Clamp `dt` (avoid huge jumps on tab-switch) and make paddle motion use `max_speed * dt`.
- [ ] Add an on-screen debug readout (or console) showing current `dt` and steps/sec.
- **Done when**: The ball moves at the same *speed* regardless of FPS, and the paddle never teleports.

### Step 03 — Fixed timestep loop (learnable + stable)
- [ ] Add a fixed `dt_fixed` (e.g., 1/120) with an accumulator.
- [ ] Step the world 0..N times per render frame based on the accumulator.
- [ ] Cap max steps per frame (avoid spiral-of-death).
- **Done when**: Motion is stable during resize/jank and the sim “feels the same” on different machines.

### Step 04 — Make core logic independent from WebGPU/WASM
- [ ] Create pure-Rust modules (no `web_sys`, no `wgpu`): `world.rs`, `discretize.rs`, `qlearn.rs`, `metrics.rs` (names can vary).
- [ ] Gate wasm-only modules with `#[cfg(target_arch = "wasm32")]` as needed so core logic builds everywhere.
- [ ] Add minimal unit tests for core logic (physics + discretization + Q update) to give newbies fast, deterministic checks.
- **Done when**: Core logic compiles and tests run without a browser.

### Step 05 — Implement `World` types + reset
- [ ] Create `rust/src/world.rs` with `World`, `Ball`, `Paddle`, `Action`, `StepOut`.
- [ ] Implement `World::new()` and `World::reset_episode()`.
- [ ] Add unit test(s) verifying reset state is in expected ranges.
- **Done when**: `World` can be created/reset deterministically.

### Step 06 — Physics Contract v1 (now that you can see motion)
- [ ] Write the **coordinate system** rules (UV \([0..1]\), origin, axes) in this file.
- [ ] Choose explicit numeric constants (paddle size, ball radius, max speed, etc.).
- [ ] Add 3 numeric examples (“given state → after one step”) for wall bounce / paddle bounce / terminal.
- **Done when**: `World::step()` rules are unambiguous, and unit tests can be written directly from the examples.

### Step 07 — Implement walls (visible + testable)
- [ ] Implement left/right/top wall collisions per the contract.
- [ ] Ensure penetration correction (snap back inside).
- [ ] Add unit tests: hitting each wall reflects velocity as expected.
- **Done when**: You can watch the ball bounce off the walls and tests pass.

### Step 08 — Add paddle collision + terminal + rewards
- [ ] Implement paddle collision (“bounce only if falling”) per the contract.
- [ ] Implement terminal condition (ball below bottom) and episode reset.
- [ ] Implement rewards: +1 bounce, -1 miss, 0 otherwise.
- **Done when**: You can keep the ball alive by moving the paddle, and misses reset the episode.

### Step 08 — Input plumbing (keyboard → WASM → current action)
- [ ] Add keyboard controls in `web/src/main.ts` (left/right; optionally space).
- [ ] Add WASM API to set current action (or store in a thread_local flag).
- [ ] In Rust update loop, read action and apply to world step.
- **Done when**: User can reliably move paddle with keys.

### Step 09 — Episode loop + basic stats (what happens when you miss)
- [ ] Track per-episode counters (steps, bounces).
- [ ] On terminal, reset episode and increment episode count.
- [ ] Log a concise episode summary periodically.
- **Done when**: Episodes end/restart and stats increment correctly.

### Step 10 — Deterministic RNG + seed (reproducible runs)
- [ ] Implement a tiny pure-Rust RNG (LCG/xorshift).
- [ ] Use it to randomize initial ball velocity/position slightly.
- [ ] Add a seed setter (constant or WASM export).
- **Done when**: Same seed produces same first N episodes.

### Step 11 — Discretization (floats → finite `state_id`)
- [ ] Create `rust/src/discretize.rs` with `Bins`, `state_id()`, `num_states()`.
- [ ] Implement binning + base-N packing.
- [ ] Unit test: `state_id` always in \[0, num_states).
- **Done when**: State encoder is stable, bounded, and changes with motion.

### Step 12 — Q-table structure + indexing
- [ ] Create `rust/src/qlearn.rs` with `QLearn { q: Vec<f32>, ... }`.
- [ ] Implement indexing `q[s * A + a]` and helpers (`argmax`, `max_q`).
- [ ] Unit test indexing correctness.
- **Done when**: Q-table reads/writes are correct and test-covered.

### Step 13 — Epsilon-greedy action selection
- [ ] Implement epsilon-greedy selection using the RNG.
- [ ] Add tests: epsilon=0 picks argmax; epsilon=1 picks random actions.
- [ ] Add a mode flag (Manual vs Agent).
- **Done when**: Agent chooses actions as expected under different epsilons.

### Step 14 — Q-learning update (Bellman update)
- [ ] Implement update rule with alpha/gamma.
- [ ] Decide terminal handling (no bootstrap on terminal).
- [ ] Add a deterministic unit test for one transition update.
- **Done when**: Update math is verified by tests.

### Step 15 — Add a mode/state machine (Play / Train / Pause / Evaluate)
- [ ] Define a single `Mode` enum in Rust (and/or mirrored in JS): `Play`, `Train`, `Paused`, `Evaluate`.
- [ ] Define “paused” semantics: sim continues, learning updates OFF; evaluate sets epsilon=0 by default.
- [ ] Add WASM exports to set mode and read current mode for UI.
- **Done when**: You can switch modes at runtime without recompiling.

### Step 16 — Training loop integration (Train mode, accelerated)
- [ ] Add `K` sim steps per render frame for fast learning feedback.
- [ ] Per step: compute s, choose a, step world, compute s', update Q, handle done.
- [ ] Keep rendering from the current world state (not per-substep) for clarity.
- **Done when**: Training shows measurable improvement (bounces/episode increases).

### Step 17 — Metrics collection (what to measure and how)
- [ ] Define metrics struct: episode length, bounces/episode, moving averages, steps/sec.
- [ ] Update metrics at episode boundaries and at fixed time intervals (avoid spam).
- [ ] Expose metrics to JS via WASM getters (or one JSON snapshot getter).
- **Done when**: You can read a stable metric snapshot from JS at any time.

### Step 18 — Metrics overlay (teach what’s happening)
- [ ] Add moving averages (bounces/episode, episode length).
- [ ] Expose metrics to JS (WASM exports) or log in a structured way.
- [ ] Render metrics in a minimal overlay panel in `web/`.
- **Done when**: A beginner can tell learning is happening without reading code.

### Step 19 — Controls panel (Train/Stop, Reset, Speed, Epsilon, Mode)
- [ ] Implement UI buttons: Train/Stop, Reset episode, Reset training.
- [ ] Implement UI controls: speed (K), epsilon slider, mode selector.
- [ ] Ensure UI reflects current mode and parameters.
- **Done when**: User can operate the experiment without touching devtools.

### Step 20 — Manual interference during training (the “teachability” feature)
- [ ] Add a “Manual override” toggle (agent action replaced by user action).
- [ ] Add an “Interference strength” option (blend/manual priority).
- [ ] Display indicator when interference is active.
- **Done when**: User can intervene during training and observe effect on outcomes.

### Step 21 — Persistence (Save/Load Q-table) + doc polish
- [ ] Implement save/load (localStorage is fine for first version).
- [ ] Include metadata (bins/action count/version) to detect mismatches.
- [ ] Update `README.md` with user-facing controls and what to expect.
- **Done when**: Learned behavior can be saved, reloaded, and validated in Eval mode.

---

## Optional appendix: only if the scaffold is broken (not part of the plan)

If you can’t see the paddle + ball at all, use `scaffold.md` to debug:
- Canvas sizing + DPR + resize handshake in `web/src/main.ts`
- `run(canvas)` entrypoint in `rust/src/lib.rs`
- Uniform upload + render loop in `rust/src/render.rs`
- Shape math in `rust/src/shader.wgsl`

---

## Notes for newbies (guardrails)

- Keep state space small at first (bins too large makes learning look “broken”).
- Prefer fixed timestep for consistent learning signals.
- Always add a “Stop training” button so you can evaluate cleanly.


