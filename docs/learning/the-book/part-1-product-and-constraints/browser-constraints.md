# Browser Constraints That Lock the Design

In native engine architecture, you can reach for threads, shared memory, and compute APIs without asking permission. In the browser, you can do the same—but only if you accept a particular set of constraints.

This chapter is not a tutorial; it is a **design filter**. Each constraint removes entire categories of architecture.

## Constraint: Work must be partitioned across workers

The main thread is for UI. If training shares the main thread, the UI stutters; if rendering shares the main thread, the UI stutters. So:

- the show match (simulation + rendering) belongs in a worker
- training belongs in a separate worker
- rollout generation can be scaled by adding more workers

## Constraint: SharedArrayBuffer requires cross-origin isolation

High-throughput copy-free sharing (weights, rollouts) is built on SharedArrayBuffer (SAB).

But SAB is only available when the page is **cross-origin isolated**, meaning:

- `Cross-Origin-Opener-Policy: same-origin`
- `Cross-Origin-Embedder-Policy: require-corp`

This constraint affects hosting, dev server config, and asset loading. It’s not optional for the “max-perf” path.

## Constraint: WebGPU must work in workers

We rely on two worker capabilities:

- **WebGPU in Dedicated Workers** (via `WorkerNavigator.gpu`)
- **OffscreenCanvas** for worker-driven rendering

This makes “render off the main thread” not merely an optimization, but a clean architectural boundary.

## Constraint: WASM threads require SAB and the right toolchain

Parallelism inside WASM can be achieved using:

- a thread pool backed by workers
- atomic memory operations via SAB
- tooling such as `wasm-bindgen-rayon` to make Rayon-style parallelism viable

This determines how rollout generation is parallelized inside a worker and how we initialize worker pools.

## Consequence: fewer messages, more shared memory

Message passing is great for:

- control messages (play/pause, hyperparameters)
- small telemetry updates (1–2 Hz)

It is not great for:

- large model weights every second
- gigabytes of rollouts per minute

Therefore, DrillSergeantHQ’s design is **SAB-first** for hot data paths.

---

**Prev:** [Goals, Non-Goals, and Dependency Policy](goals-non-goals-dependency-policy.md)  
**Next:** [Part II — System Architecture](../part-2-system-architecture/README.md)


