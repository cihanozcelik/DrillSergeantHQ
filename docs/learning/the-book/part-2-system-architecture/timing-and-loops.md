# Timing and Loops: Show vs Training

When readers get lost, it’s usually because they can’t tell which loop they’re in.

DrillSergeantHQ has two big loops running concurrently:

- the **show loop** (simulation + rendering + inference)
- the **training loop** (rollout consumption + PPO updates + checkpoint publish)

The system feels stable only when we treat these loops as **budgets**.

## The show loop (Render/Eval Worker)

The show loop is a classic real-time “game” loop with three cadences:

1. **Simulation tick** (fixed): e.g. 120 Hz
2. **Render** (vsync-ish): e.g. 60 FPS
3. **Action selection** (coarser): e.g. 10–30 Hz via action repeat

### Fixed timestep simulation (canonical pattern)

The show env advances by a fixed \(\Delta t\) regardless of rendering jitter:

```text
accum += frame_dt
while accum >= sim_dt:
  step_sim(sim_dt)
  accum -= sim_dt
render(interpolate?)
```

This gives determinism (critical for debugging) and keeps physics stable.

### Action repeat (why inference is not every sim tick)

Inference is expensive and often unnecessary at 120 Hz. Instead:

- choose an action every \(k\) sim steps
- apply the action for \(k\) steps (action repeat)

This improves throughput and makes the hot-swap story cleaner: policy decisions happen at a known cadence.

### Weight hot-swap cadence (human-visible)

Weight updates are “large” compared to control messages, but “small” compared to rollouts. A good default is:

- publish checkpoints after each PPO update, and/or at ~1 Hz
- hot-swap on **version change** or on a timer (e.g. every 1000 ms)

The show match does not restart; only the policy’s weight pointer changes.

## The training loop (Trainer Worker)

Training is a throughput loop. It does not care about 60 FPS; it cares about **steps per second** and **updates per second**.

Conceptually:

```text
while training:
  rollouts = read_from_ring_buffer()
  adv, returns = compute_gae(rollouts)
  for epoch in 1..E:
    for minibatch in shuffle(rollouts):
      ppo_update(minibatch)
  publish_checkpoint()
```

The key is separation:

- rollout workers keep producing (CPU bound)
- trainer updates in bursts (often GPU bound)

## Scheduling: avoiding interference

Even with separate workers, the GPU is a shared resource. Two practical rules:

- **Prefer smooth rendering**: if rendering and training contend, the show worker wins.
- **Batch training**: large kernels less frequently tends to be better than constant tiny kernels.

Some systems implement explicit “GPU timeslices” (render frame, then train). DrillSergeantHQ can evolve toward that if needed, but v1’s primary protection is **worker separation** and sane publish cadence.

## The three budgets you should track

1. **Show FPS** (and frame time histogram)
2. **Rollout SPS** (steps per second across all rollout workers)
3. **Update UPS** (PPO updates per second, or seconds per update)

If the system feels “wrong,” one of these budgets is usually the reason.

---

**Prev:** [Worker Topology and Responsibilities](worker-topology.md)  
**Next:** [Protocols: Messages and Contracts](protocols.md)



