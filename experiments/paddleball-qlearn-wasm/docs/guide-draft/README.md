# PaddleBall Q-learning — Step-by-step Implementation Guide

This guide is the **implementation companion** to:
- `experiments/paddleball-qlearn-wasm/docs/plan.md`

**Important**: this experiment is **not deep learning** (no neural network). It’s **tabular Q-learning** (a Q-table).

## How this guide is organized

- One file per plan step: `01-...md` through `21-...md` (plan has 22 steps; guide 21 = plan step 22)
- Each file is broken down by **todo IDs** (e.g. `08-01`) so we can review precisely.
- Every todo includes:
  - **What to change**
  - **Technique** (how)
  - **Why** (reason/tradeoff)
  - **Code** (diff or full file for new modules)

## Index (mirrors `plan.md`)

- [Step 01 — Make the ball move](01-make-the-ball-move.md)
- [Step 02 — Make the paddle move](02-make-the-paddle-move.md)
- [Step 03 — Fixed timestep accumulator](03-fixed-timestep-accumulator.md)
- [Step 04 — Keyboard input](04-keyboard-input.md)
- [Step 05 — Minimal episode mechanics](05-minimal-episode-mechanics.md)
- [Step 06 — Physics contract (walls)](06-physics-contract-walls.md)
- [Step 07 — Implement wall bounces](07-implement-wall-bounces.md)
- [Step 08 — Paddle collision](08-paddle-collision.md)
- [Step 09 — Terminal + auto reset](09-terminal-and-auto-reset.md)
- [Step 10 — Reward signal](10-reward-signal.md)
- [Step 11 — Extract core logic to `world.rs`](11-extract-world-core.md)
- [Step 12 — First unit tests](12-first-unit-tests.md)
- [Step 13 — Deterministic RNG + seeding](13-deterministic-rng.md)
- [Step 14 — Discretization (state_id)](14-discretization-state-id.md)
- [Step 15 — Q-table storage + helpers](15-qtable-storage.md)
- [Step 16 — Epsilon-greedy + agent play](16-epsilon-greedy-agent-play.md)
- [Step 17 — Q-learning update](17-qlearning-update.md)
- [Step 18 — Training loop](18-training-loop.md)
- [Step 19 — Learning progress metrics](19-learning-metrics.md)
- [Step 20 — Controls panel](19-learning-metrics.md)
- [Step 21 — Manual interference](20-manual-interference.md)
- [Step 22 — Save/Load Q-table + doc polish](21-save-load-and-polish.md)


