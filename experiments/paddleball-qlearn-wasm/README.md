# PaddleBall Q-learning (WASM + wgpu)

This is a **standalone experiment** inside the DrillSergeantHQ repo:
**Rust + WASM + WebGPU (wgpu)** + a tiny RL environment (PaddleBall + tabular Q-learning).

**Important**: this experiment is **not deep learning** (no neural network). It’s classic **tabular Q-learning**.

The end goal is a beginner-friendly “paddle keeps a ball alive” environment that evolves into:
- deterministic-ish fixed timestep physics
- sparse reward (`+1` bounce, `-1` terminal)
- discretized state
- tabular Q-learning

## Status

- ✅ **Rendering scaffold**: paddle + ball draw correctly, resize is stable, aspect is preserved

## How to run

- **Prereqs**: Rust, Node.js, `wasm-pack`, `cargo-watch`
- **Install + dev**:

```bash
cd experiments/paddleball-qlearn-wasm
npm install
npm run dev
```

Open the URL printed by Vite.

## Start here

- **Read the plan**: `docs/plan.md` (what you’ll build, with todo IDs like `08-01`)
- **Follow the implementation guide (draft)**: `docs/guide-draft/README.md` (how to implement each step)
- **If you’re confused about WebGPU/WASM/rendering**: `docs/scaffold.md` (how the scaffold works)

## Learning Resources

- **`docs/scaffold.md`**: the newbie explainer of the current scaffold (render pipeline, TS↔WASM flow, shader, uniforms).
- **`docs/plan.md`**: the official, testable step-by-step development plan (what to build next).
- **`docs/guide-draft/README.md`**: step-by-step implementation guide (draft; not yet proven).

## Layout

```
experiments/paddleball-qlearn-wasm/
  docs/
    plan.md
    scaffold.md
    guide/
```