# Deterministic Fixed Timestep Simulation

The show match has a simple job: advance the world in a way that feels stable and is easy to debug.

The core technique is the same one used in many real-time games:

> **Simulate at a fixed timestep. Render at a variable timestep.**

## Why fixed timestep is the right default

Fixed timestep gives you:

- **determinism (practical)**: same seed + same inputs → same outcomes
- **stable physics**: integration doesn’t change with frame jitter
- **debuggability**: you can “replay” a run by replaying input events

## The canonical accumulator loop

```text
sim_dt = 1/120
accum = 0
last_time = now()

on_frame():
  t = now()
  frame_dt = clamp(t - last_time, 0, 0.25)
  last_time = t
  accum += frame_dt

  while accum >= sim_dt:
    step_sim(sim_dt)
    accum -= sim_dt

  render()
```

### Notes that matter in practice

- **Clamp `frame_dt`**: if a tab stalls or the machine hiccups, you don’t want to simulate “20 seconds of physics” in one burst.
- **Bound the while-loop**: in extreme cases, cap the number of sim steps per frame to avoid death spirals.
- **Keep `step_sim` allocation-free**: the show loop must be predictable.

## Determinism: what we mean (and what we don’t)

In the browser, “perfect determinism across all GPUs/CPUs” is not realistic. But we can get **useful determinism**:

- determinism within the same browser/hardware
- determinism for simulation logic (CPU, same floating point behavior)
- determinism for control flow and RNG usage

**Contract**

- simulation RNG is explicit and seedable
- simulation uses a fixed `sim_dt`
- user inputs are timestamped and applied deterministically (e.g., at the next sim tick boundary)

## Show env vs training env

The show env is **one** environment instance, stepped and rendered.

Training envs are **many** instances, stepped in batches for throughput.

They share simulation logic, but their loops differ:

- show loop is budgeted to feel smooth
- training loop is budgeted for steps/sec

## Failure modes

- **Spiral of death** (can’t catch up):  
  - symptom: frame time spikes, simulation “lags behind”
  - defense: cap steps per frame, degrade gracefully (slow-mo) rather than hanging

- **Non-reproducible bugs**:  
  - symptom: “sometimes the ball clips through the wall”
  - defense: fixed timestep + seed + replayable inputs

---

**Prev:** [Part IV — The Show Environment](README.md)  
**Next:** [Action Selection and “No Teleport Jumps”](action-selection.md)


