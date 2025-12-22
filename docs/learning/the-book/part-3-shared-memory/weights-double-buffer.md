# Weights Publishing: Double Buffer + Atomics

Weights publishing is the bridge between “training is happening” and “the show match gets smarter.”

The requirements are deceptively strict:

- the Trainer must publish frequently
- the Render/Eval worker must adopt new weights without copying megabytes via messages
- adoption must be safe, deterministic, and low overhead
- the show match must not stall while weights are being written

This is exactly what a **double buffer** is for.

## The pattern: write inactive, then flip

We maintain two equal-sized weight buffers:

- `weights_A` (Float32Array view on SAB)
- `weights_B` (Float32Array view on SAB)

And a tiny control buffer:

- `weights_ctrl` (Int32Array view on SAB, accessed via Atomics)

At any time, one weight buffer is active (readable by the show match), and the other is inactive (writable by the trainer).

## Control layout (Int32 indices)

**Contract (v1)**

`weights_ctrl[i32]` fields:

- `[0] active_idx`  
  - **meaning**: 0 => A is active, 1 => B is active  
  - **writer**: Trainer  
  - **reader**: Render/Eval  
  - **ops**: `Atomics.load/store`

- `[1] version`  
  - **meaning**: monotonic counter incremented on publish  
  - **writer**: Trainer  
  - **reader**: Render/Eval  
  - **ops**: `Atomics.load/add`

- `[2] shape_hash` (optional but recommended)  
  - **meaning**: detects architecture/layout changes  
  - **writer**: Trainer  
  - **reader**: Render/Eval  
  - **ops**: `Atomics.load/store`

- `[3] step_counter` (optional)  
  - **meaning**: “global training steps” or “updates completed”  
  - **writer**: Trainer  
  - **reader**: UI / Render/Eval  
  - **ops**: `Atomics.load/store`

## Publish protocol (Trainer)

Pseudo-code:

```text
active = Atomics.load(ctrl, ACTIVE_IDX)
inactive = 1 - active

dst = (inactive == 0) ? weights_A : weights_B
write_all_weights(dst)           // contiguous write

Atomics.store(ctrl, ACTIVE_IDX, inactive)
Atomics.add(ctrl, VERSION, 1)
```

Key property: the show match never reads the buffer currently being written.

## Hot-swap protocol (Render/Eval)

Pseudo-code:

```text
v = Atomics.load(ctrl, VERSION)
if v != last_v:
  idx = Atomics.load(ctrl, ACTIVE_IDX)
  src = (idx == 0) ? weights_A : weights_B
  policy.set_weights_view(src)   // pointer swap (no copy)
  last_v = v
```

This can run:

- on a timer (e.g. every 1000 ms), or
- on version change checks placed in the show loop

## Memory ordering: what’s “guaranteed”?

JavaScript Atomics establish ordering for atomic operations on the control buffer.

The important discipline is:

- **Trainer**: write weights first, then atomically flip + bump version.
- **Render/Eval**: read version, then read active index, then switch pointer.

In practice, this pattern is widely used for safe publication of data across threads.

## Failure modes and defenses

- **Shape mismatch** (trainer changed network):  
  - defense: `shape_hash` check; if mismatch, refuse hot-swap and request a reset/re-init.

- **Torn publish** (buggy trainer publishing before write completes):  
  - defense: publish protocol discipline + tests; optionally add a `publish_state` flag in ctrl for debug builds.

- **Too-frequent swaps causing “snap” behavior**:  
  - defense: set a sane cadence; optionally implement blending in action distribution (advanced topic in Part IV).

## Why this contract is worth the ceremony

With this contract:

- training can publish every second
- show match can adopt instantly
- the UI stays smooth
- you never ship megabytes through message channels

This is the heart of “live improvement.”

---

**Prev:** [Part III — Shared Memory Contracts](README.md)  
**Next:** [Rollout Transport: Ring Buffer (N→1)](rollout-ring-buffer.md)


