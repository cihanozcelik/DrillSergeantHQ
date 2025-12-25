# PaddleBall Q-learning (WASM + wgpu)

This is a **standalone experiment** inside the DrillSergeantHQ repo:
**Rust + WASM + WebGPU (wgpu)** + a tiny RL environment (PaddleBall + Q-learning).

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

## Learning Resources

- **`scaffold.md`**: the newbie explainer of the current scaffold (render pipeline, TS↔WASM flow, shader, uniforms).
- **`hypothetical-implementation.md`**: pre-thinking / implementation notes (a plausible plan; **not** the official tutorial).
- **`tutorial.md`**: placeholder for the future real step-by-step tutorial.

## Layout

```
experiments/paddleball-qlearn-wasm/
  tutorial.md
  hypothetical-implementation.md
  scaffold.md
```