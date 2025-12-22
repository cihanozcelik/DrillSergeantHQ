# What DrillSergeantHQ Is

DrillSergeantHQ is a **browser-only agent training playground**. It is built to make reinforcement learning feel *physical* and *visible*:

- You can run a simple arena match as a demo.
- You can press **Train** and watch behavior improve in the same session.
- You can still use normal controls: play/pause, speed, reset, “poke the world,” and evaluate.

This implies a split-brain system:

- One runtime owns the **experience** (the show match, the rendering, the input).
- Another runtime owns the **learning** (rollouts, updates, checkpoints, telemetry).

And crucially: these runtimes are connected by **contracts**, not by “shared mutable application state.”

## The core technical identity

The project’s identity is not a pile of frameworks. It’s a set of decisions:

- **Rust in the core**: simulation, RL math, training control logic.
- **WASM for portability**: the entire system runs on-device, in the browser.
- **WebGPU for throughput**: rendering and training compute can share the same modern GPU API.
- **Workers for parallelism**: show and training run concurrently; training scales with CPU cores.

## The north star: “continuous improvement UX”

Most RL demos hide learning behind minutes of waiting, then reveal a suddenly improved agent. DrillSergeantHQ aims for the opposite:

- training produces periodic improvements
- improvements are published as checkpoints
- the show match hot‑swaps weights on a schedule (or on version change)
- the world keeps running; only the “decision function” changes

If you remember one sentence from this chapter, make it this:

> DrillSergeantHQ is built around hot‑swapping policies into a live simulation without breaking continuity.

---

**Prev:** [Part I — The Product and the Constraints](README.md)  
**Next:** [The “Live Improvement” Promise](live-improvement-promise.md)


