# Shared Memory Strategy (SAB First)

If you remember one performance rule for this project, remember this:

> If it’s hot and large, it must be shared and structured.

DrillSergeantHQ uses SharedArrayBuffer (SAB) to keep the highest-bandwidth channels copy-free:

- **weights publishing** (Trainer → Render/Eval)
- **rollout transport** (Rollout workers → Trainer)

Messages still exist, but they are deliberately small.

## Why SAB instead of postMessage?

For small commands, `postMessage` is perfect.

For continuous bulk data, it’s the wrong tool:

- it encourages copy-heavy designs
- it creates backpressure in unpredictable places
- it makes latency spikes common (especially when GC gets involved)

SAB flips the model:

- allocate once
- write in place
- communicate with atomic indices/versioning

## The two SAB patterns used in DrillSergeantHQ

### Pattern A: Double buffer for weights (1 producer → 1 consumer)

Weights have a “latest version” meaning. We don’t need a queue; we need a *stable snapshot*.

So we use:

- `weights_A` and `weights_B` (two equally-sized float buffers)
- a tiny `ctrl` buffer containing:
  - `active_idx` (0 or 1)
  - `version` (monotonic)
  - optional: `shape_hash`, `step_counter`

Publish is “write then flip.” Read is “check version then switch pointer.”

### Pattern B: Ring buffer for rollouts (N producers → 1 consumer)

Rollouts are a stream. You want high throughput and minimal overhead:

- many rollout workers write experiences
- the trainer reads and consumes them

This is a classic producer/consumer problem. The performance trick is to make the “record format” SoA and make the control plane atomic and tiny.

## Design stance: SoA wins

For hot data:

- prefer **Structure of Arrays** (SoA) over Array of Structs (AoS)
- prefer contiguous buffers over nested objects
- prefer fixed-size records over “flexible” objects

This is true in Rust, in WASM, and on the GPU.

## Contract: shared memory must be self-describing enough

SAB is raw bytes. If you don’t define layout/versioning, you will ship “Heisenbugs.”

**Contract**

- Every SAB-backed structure has:
  - a layout document (offsets/strides/types)
  - a version or shape hash
  - a strategy for upgrades (compat or hard fail)
- Every atomic field is documented with:
  - who writes it
  - who reads it
  - which atomic operation is required (`store`, `load`, `add`, etc.)

Part III is where we make this concrete.

---

**Prev:** [Protocols: Messages and Contracts](protocols.md)  
**Next:** [Part III — Shared Memory Contracts](../part-3-shared-memory/README.md)


