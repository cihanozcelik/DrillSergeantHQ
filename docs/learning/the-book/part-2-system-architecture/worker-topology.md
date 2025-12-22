# Worker Topology and Responsibilities

If you want DrillSergeantHQ to feel like a real-time game *and* a training system, you must prevent training from stealing the show.

The browser gives us a clean way to do that: **workers**. Each worker becomes a “mini-program” with a narrow responsibility and tight performance budget.

## The topology (v1)

```text
Main Thread (UI)  <---- small msgs ---->  Render/Eval Worker (show match)
        |
        | small msgs
        v
Trainer Worker  <---- SAB rollouts ----  Rollout Worker Pool (N)
   |
   +---- SAB weights (double buffer) ---> Render/Eval Worker
```

This topology is not primarily about parallelism. It’s about **isolation**:

- UI stays responsive.
- The show loop stays smooth.
- Training can run aggressively without harming the experience.

## Main Thread (React UI): the product shell

**Owns:**

- DOM/UI controls (Train/Stop, Play/Pause, speed, reset)
- input capture (mouse/keyboard/touch) and packaging
- displaying metrics at human cadence (typically 1–2 Hz)

**Does not own:**

- simulation stepping
- WebGPU rendering
- training math

**Rule:** the UI thread sends **commands**, not “state diffs of the world.”

## Render/Eval Worker: the show match

**Owns:**

- the **single show environment** (1 instance)
- fixed-timestep simulation and rendering
- action selection + inference for the show agent(s)
- hot-swapping policy weights

**Budget mindset:**

- sim ticks must be deterministic and bounded
- rendering must hit a stable target (e.g., 60 FPS)
- action selection cadence is explicit (e.g., 10–30 Hz)

**Rule:** the show environment is **evaluation-only**. It must not become a hidden training dependency.

## Trainer Worker: the learner and publisher

**Owns:**

- consuming rollouts
- computing GAE / returns
- running PPO updates
- checkpoint publishing (weights)
- telemetry aggregation

**Why a dedicated worker?**

Training is bursty: kernels, reductions, and minibatch loops create unpredictable CPU/GPU load. Isolation prevents that load from harming the show match.

## Rollout Worker Pool (N): throughput factory

**Owns:**

- stepping thousands of headless env instances
- producing experience continuously
- writing to a shared ring buffer

The rollout pool scales with cores. If your device has 12 physical cores, you can spend most of them here—while still keeping the UI and show match smooth.

## Why not “one worker to rule them all”?

You *can* put everything in one worker and use internal scheduling. But you lose the browser’s strongest architecture tool: **hard isolation**.

The separate-worker design also forces a discipline that’s good for teams:

- every boundary has a message schema or shared-memory ABI
- every hot path has an explicit layout
- every subsystem can be tested and profiled independently

## Failure modes (and how topology prevents them)

- **UI stutter**: avoided by keeping sim/render/training off the main thread.
- **Show stutter**: avoided by keeping training in a different worker.
- **Rollout starvation**: avoided by scaling rollout workers separately from training.
- **Silent ABI drift**: mitigated by writing down contracts (Part III).

---

**Prev:** [The “WTF Map”: One Tick Through the Whole System](wtf-map.md)  
**Next:** [Timing and Loops: Show vs Training](timing-and-loops.md)


