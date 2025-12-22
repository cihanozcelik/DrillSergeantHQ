# Goals, Non-Goals, and Dependency Policy

DrillSergeantHQ is a *learning project* in the best sense: it rebuilds the fundamentals so the system is understandable, debuggable, and fast.

## Goals

- **Browser-only**: no native install; everything runs on-device.
- **Max performance**: saturate CPU cores for rollouts; use GPU for rendering and training compute.
- **Continuous improvement UX**: hot‑swap policies so behavior improves without restarting.
- **Normal controls**: play/pause, speed, reset.
- **Training isolation**: show match and user input do not contaminate training data.

## Non-goals

- Shipping a framework-dependent RL stack (e.g., a full TF.js training pipeline).
- Server-side training or cloud compute.
- Multi-tab/multi-user synchronization.

## Dependency policy: what we allow vs. what we build

The project draws a bright line:

### Allowed: protocol glue

Dependencies are allowed when they *expose the platform*:

- WebGPU access/abstraction: `wgpu` (or equivalent)
- WASM ↔ JS interop: `wasm-bindgen`, `js-sys`, `web-sys`
- minimal bundling/build glue

### Built in-house: product core

We intentionally build the system’s core value:

- simulation (deterministic stepping, collisions, SoA batching)
- RL math (PPO/GAE)
- model inference (tiny MLP, rollout-side)
- training orchestration (rollout scheduling, updates, checkpoint publishing)
- WGSL kernels and binding layouts
- shared-memory ABIs and worker bootstrapping

### Disallowed: “you imported the project”

- full game engines/physics engines
- full ML frameworks
- black-box PPO implementations that remove control over layout/scheduling/validation

## Why this strictness is healthy

The purpose is not purity. The purpose is **understanding**.

When the RL update blows up, you should be able to trace it to:

- an ABI mismatch
- a kernel bug
- a numerical stability issue
- a mistaken assumption in the sim/reward

Not to “some library did something weird.”

---

**Prev:** [The “Live Improvement” Promise](live-improvement-promise.md)  
**Next:** [Browser Constraints That Lock the Design](browser-constraints.md)


