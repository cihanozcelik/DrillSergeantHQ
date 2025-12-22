# Roadmap (Phases)

The technical design describes the project as an early-stage system with a performance-oriented implementation plan.

This appendix captures that plan in “definition of done” language, so readers can understand what exists today and what is intended.

## Phase A — Foundation (COOP/COEP + workers + WASM threads)

**Goal:** establish the runtime topology and the permissions/security posture required for performance.

Definition of Done:

- `crossOriginIsolated === true`
- rendering runs in a worker (OffscreenCanvas)
- Rayon-style parallelism works in a worker (WASM threads)
- SAB control blocks initialize reliably

## Phase B — CPU sim + show rendering

**Goal:** show match loop feels like a real game.

- deterministic fixed timestep sim
- rendering in Render/Eval worker
- baseline AI (random/scripted)
- UI controls (play/pause/speed/reset)

## Phase C — Rollout engine + RL data structures

**Goal:** build the throughput factory.

- SoA env batches
- rollout buffers and schemas
- GAE + returns implementation

## Phase D — PPO update (GPU compute)

**Goal:** accelerate learning enough for live feedback.

- WebGPU compute foundation in Trainer worker
- forward/backward kernels and Adam update
- checkpoint publishing via SAB double buffer

## Phase E — Show hot-swap + “live improvement” UX

**Goal:** the product promise is real.

- periodic hot-swaps without restarting
- optional blending to reduce snap
- visible improvement within 30–120 seconds (target)

## Phase F — Self-play pool + stabilization

**Goal:** improvement stays meaningful and stable.

- snapshot pool
- opponent sampling
- win-rate EMA

## Phase G — GPU simulation backend (optional)

**Goal:** max-performance path once correctness is proven.

- WGSL env stepping kernels
- behavioral verification vs CPU sim (validation harness)

---

**Prev:** [Math Cheat Sheet](math-cheat-sheet.md)  
**Next:** [Author Notes (internal)](../_author/README.md)


