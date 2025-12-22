## Tutorials by Topic (Standalone Articles)

This folder contains **standalone tutorials** on technical topics (RL, simulation, browser systems, and performance engineering). They are intentionally written to be understandable **without any project context**.

### How to use this folder

- If you’re new to the domain, start with the **foundations** (action spaces, PPO/GAE, backprop).
- If you’re building browser performance systems, focus on **workers, shared memory, and build/deploy headers**.
- If you’re debugging correctness, read **determinism** and **validation mode** early.

---

## Index

- **Action spaces and control**
  - [`action-space-design.md`](./action-space-design.md): What an action space is, discrete vs continuous choices, action repeat, and how to design controls that are learnable and physically valid.

- **Optimization**
  - [`adam-optimizer.md`](./adam-optimizer.md): Adam explained from first principles—moments, bias correction, AdamW, tuning, and common mistakes.
  - [`training-schedulers.md`](./training-schedulers.md): How to schedule learning rates and other training knobs over time (warmup, cosine decay, exploration schedules).

- **Neural network fundamentals**
  - [`mlp-backpropagation.md`](./mlp-backpropagation.md): MLPs and backpropagation with clear shapes, derivations, and an implement-from-scratch workflow.

- **RL math and training loops**
  - [`gae-math.md`](./gae-math.md): GAE(\(\lambda\)) explained: TD error, bias–variance tradeoff, and a compute-it-from-rollouts recipe.
  - [`ppo-gae-loops.md`](./ppo-gae-loops.md): The full PPO training loop: what to store, how to compute ratios/advantages, and how epochs/minibatches fit together.
  - [`reward-shaping-potential.md`](./reward-shaping-potential.md): Reward shaping without reward hacking, including potential-based shaping and a responsible workflow.
  - [`rollout-orchestration.md`](./rollout-orchestration.md): How to build rollout pipelines (vectorization, multi-worker producers, bottlenecks, correctness pitfalls).
  - [`parallel-ppo-simulation.md`](./parallel-ppo-simulation.md): Scaling PPO rollouts across many cores: freshness, correlation, communication cost, and architecture patterns.

- **Self-play**
  - [`self-play-theory.md`](./self-play-theory.md): Why self-play works, why it fails, and the core stabilization ideas.
  - [`self-play-pool.md`](./self-play-pool.md): Opponent pools as a “league” system: snapshot strategy, sampling strategy, pruning, and evaluation.

- **Simulation and physics**
  - [`physics-determinism.md`](./physics-determinism.md): Fixed timesteps, determinism, integration basics, and the common sources of nondeterminism.
  - [`soccer-sim-logic.md`](./soccer-sim-logic.md): How to structure a minimal 2D arena physics sim: semi-implicit Euler, circle/wall and circle/OBB collision concepts, and stability guardrails.

- **Observability and debugging**
  - [`training-metrics.md`](./training-metrics.md): What to log during training: outcome vs optimization metrics, PPO health metrics, throughput, and early warning signs.
  - [`validation-mode-cross-checking.md`](./validation-mode-cross-checking.md): How to debug fast systems using reference paths, tolerances, checkpoints, and divergence localization.

- **Browser performance systems (Workers, memory, storage)**
  - [`off-main-thread-rendering.md`](./off-main-thread-rendering.md): OffscreenCanvas and moving rendering to a worker with input/resizing patterns and pitfalls.
  - [`worker-request-animation-frame.md`](./worker-request-animation-frame.md): Building worker frame loops with rAF when available and safe fixed-timestep fallbacks.
  - [`shared-array-buffer-atomics.md`](./shared-array-buffer-atomics.md): Shared memory concurrency: SAB vs ArrayBuffer, Atomics, and a basic ring-buffer pattern.
  - [`soa-layouts.md`](./soa-layouts.md): Structure-of-Arrays vs Array-of-Structs: performance intuition, conversion example, and when each layout wins.
  - [`memory-alignment-layout.md`](./memory-alignment-layout.md): Alignment, padding, offsets/stride, ABI contracts, and how to avoid “random corruption” across boundaries.
  - [`opfs-storage.md`](./opfs-storage.md): OPFS as a file-like storage backend in the browser: concepts, workflows, and product concerns.

- **WebGPU and shaders**
  - [`webgpu-dedicated-workers.md`](./webgpu-dedicated-workers.md): Running WebGPU in a dedicated worker: architecture, data flow, robustness, and fallbacks.
  - [`wgsl-kernels.md`](./wgsl-kernels.md): Compute kernels in WGSL: mapping work, memory bottlenecks, reductions, and an optimization checklist.

- **Build and deployment**
  - [`build-pipeline-tooling.md`](./build-pipeline-tooling.md): How modern builds ship when you have workers/WASM: entrypoints, artifact layout, caching, and troubleshooting.
  - [`coop-coep-isolation.md`](./coop-coep-isolation.md): COOP/COEP and cross-origin isolation: why it exists, what breaks, and how to deploy safely.

- **Fallbacks and compatibility**
  - [`cpu-fallback-strategy.md`](./cpu-fallback-strategy.md): Designing CPU and quality fallbacks: capability detection, ladders, UX honesty, and shipping checklists.
  - [`wasm-threads-rayon.md`](./wasm-threads-rayon.md): WebAssembly threading with Rayon: shared memory requirements, worker-backed thread pools, and pitfalls.


