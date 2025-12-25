# PaddleBall Q-learning — Step-by-step Implementation Guide

This guide is the **implementation companion** to:
- `experiments/paddleball-qlearn-wasm/docs/plan.md`

**Important**: this experiment is **not deep learning** (no neural network). It’s **tabular Q-learning** (a Q-table).

## How this guide is organized

- One file per plan step: `01-...md` through `22-...md`
- Each file is broken down by **todo IDs** (e.g. `08-01`) so we can review precisely.
- Every todo includes:
  - **What to change**
  - **Technique** (how)
  - **Why** (reason/tradeoff)
  - **Code** (diff or full file for new modules)

## Index (mirrors `plan.md`)

- [Step 01 — Make the ball move](01-make-the-ball-move.md)
- [Step 02 — Make the paddle move](02-make-the-paddle-move.md)
- [Step 03 — Real time delta + safety clamps](03-real-time-delta-and-clamps.md)
- [Step 04 — Fixed timestep accumulator](04-fixed-timestep-accumulator.md)
- [Step 05 — Keyboard input](05-keyboard-input.md)
- [Step 06 — Minimal episode mechanics](06-minimal-episode-mechanics.md)
- [Step 07 — Physics contract (walls)](07-physics-contract-walls.md)
- [Step 08 — Implement wall bounces](08-implement-wall-bounces.md)
- [Step 09 — Paddle collision](09-paddle-collision.md)
- [Step 10 — Terminal + auto reset](10-terminal-and-auto-reset.md)
- [Step 11 — Reward signal](11-reward-signal.md)
- [Step 12 — Extract core logic to `world.rs`](12-extract-world-core.md)
- [Step 13 — First unit tests](13-first-unit-tests.md)
- [Step 14 — Deterministic RNG + seeding](14-deterministic-rng.md)
- [Step 15 — Discretization (state_id)](15-discretization-state-id.md)
- [Step 16 — Q-table storage + helpers](16-qtable-storage.md)
- [Step 17 — Epsilon-greedy + agent play](17-epsilon-greedy-agent-play.md)
- [Step 18 — Q-learning update](18-qlearning-update.md)
- [Step 19 — Training loop](19-training-loop.md)
- [Step 20 — Learning progress metrics](20-learning-metrics.md)
- [Step 21 — Manual interference](21-manual-interference.md)
- [Step 22 — Save/Load Q-table + doc polish](22-save-load-and-polish.md)


