# Rollout Transport: Ring Buffer (N→1)

Rollouts are the system’s firehose.

Unlike weights (latest snapshot), rollouts are a **stream**:

- multiple producers (rollout workers)
- one consumer (trainer)
- huge volume, continuous, latency-tolerant

The goal is not “perfect delivery.” The goal is **stable throughput**. Dropping data under extreme pressure can be acceptable as long as it’s observable.

## Design goals

- **Copy-free**: producers write directly into SAB
- **Low overhead**: minimal atomic operations per record/chunk
- **Batch-friendly**: records are structured for SIMD/GPU-friendly consumption
- **Observable backpressure**: when the buffer is full, we can measure drops

## One important choice: what is a “record”?

There are two good strategies:

1. **Chunked records**: each producer reserves a large chunk (e.g., one rollout of shape \(T \times N\)) and writes it in bulk.  
2. **Fixed records**: each step is a fixed-size record; producers push step-by-step.

For performance, DrillSergeantHQ leans toward **chunked records**:

- fewer atomics
- contiguous writes
- easier minibatching for PPO

## Contract: the control plane is atomic and tiny

We model the ring buffer with a control SAB:

**Contract (conceptual fields)**

- `head`: write cursor (monotonic)
- `tail`: read cursor (monotonic)
- `capacity`: size in “records” or “chunks”
- optional: `dropped`: counter incremented when producers can’t write

In practice, you’ll often store `head` and `tail` as 32-bit indices and wrap modulo capacity.

## A practical N→1 pattern: multi-producer reservation

Multi-producer ring buffers are tricky because each producer must reserve space without colliding.

The usual pattern is:

- producers atomically `fetch_add` a reservation counter
- each producer computes its write offset from the returned reservation
- consumer reads in order using `tail`

But you must also handle the “buffer full” condition. There are a few approaches:

- **Drop-on-full**: if head - tail exceeds capacity, drop and increment `dropped`.
- **Backoff/spin**: producers spin (usually bad in the browser).
- **Per-producer lanes**: each producer has its own queue; trainer multiplexes (often simplest).

### Recommended v1 approach (simple and robust): per-producer lanes

To keep correctness and implementation complexity sane in v1:

- allocate one ring buffer per rollout worker (each is 1→1)
- trainer reads from all rings in round-robin or priority order

This avoids multi-producer atomic contention and simplifies debugging.

If/when we need max scale, we can move to a true multi-producer ring with careful reservation logic.

## Data layout: SoA inside each chunk

Inside a chunk (e.g., one rollout window), prefer SoA arrays:

- `obs[t][env]`
- `act[t][env]`
- `rew[t][env]`
- `done[t][env]`
- `logp[t][env]`
- `value[t][env]`

This layout:

- streams well in memory
- is friendly to GPU compute
- makes reductions and normalization straightforward

## Failure modes and observability

- **Buffer full**:  
  - behavior: drop chunk or overwrite oldest (choose explicitly; don’t “accidentally” do either)
  - observability: increment `dropped`, export as telemetry

- **ABI mismatch** (trainer expects different strides than producer wrote):  
  - behavior: hard fail in dev builds; shape hash/version checks in prod builds

- **Producer starvation** (trainer too slow):  
  - behavior: drops rise; training becomes noisier
  - fix: increase capacity, reduce rollout volume, optimize trainer, or scale down envs per worker

## Why this chapter is “conceptual” on purpose

The exact byte layout depends on the model and observation encoding.

So this chapter defines:

- the **invariants**
- the **failure behavior**
- the **shape of the solution**

Appendix “Buffer Layouts (Draft)” is where we pin down offsets/strides once the observation/action schema is finalized.

---

**Prev:** [Weights Publishing: Double Buffer + Atomics](weights-double-buffer.md)  
**Next:** [ABI Mindset: Layouts, Alignment, and Versioning](abi-and-versioning.md)


