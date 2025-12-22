# ABI Mindset: Layouts, Alignment, and Versioning

Shared memory turns “software bugs” into “systems bugs.”

When two runtimes (TS in a worker, Rust in WASM) share raw bytes, you have an ABI—whether you admit it or not.

This chapter teaches the mindset that prevents the most painful class of failures in DrillSergeantHQ.

## What “ABI” means here

ABI is not just “C calling conventions.” Here it means:

- the exact byte layout of shared buffers
- the meaning of each field
- the rules about who can write and who can read
- the upgrade story when layouts change

## Alignment is not optional

Even in JS, typed arrays enforce alignment rules implicitly:

- `Int32Array` indexes are 4-byte aligned
- `Float32Array` indexes are 4-byte aligned

In Rust/WASM, alignment affects:

- correctness (misaligned loads can trap or silently slow down)
- performance (SIMD and coalesced access patterns)

**Rule:** design shared buffers with explicit alignment, and document it.

## Versioning strategies

You need a clear policy for when a buffer layout changes.

### Strategy A: strict version (hard fail)

- buffer has a `version` field
- if consumer sees mismatch, it refuses to run

This is excellent for dev builds and early-stage iteration.

### Strategy B: shape hash (detect “same shape”)

- compute a stable hash of layout + network shape
- store it in ctrl
- consumer verifies before reading

This is great for weights and other “structured blobs.”

### Strategy C: compatible evolution (rare, expensive)

- maintain backward compatible decoding across versions
- only worth it when you need long-lived artifacts across releases

In v1, prefer strict versioning and clear failure.

## Contract: failure must be explicit

The worst failure mode is “it kinda works but is subtly wrong.”

**Contract**

- if shape/version mismatches, fail loudly (log + stop hot-swap, or require reset)
- never interpret unknown bytes “as if” they were the old layout

## Document layouts like an engine would

Every shared buffer should have a layout doc that includes:

- a diagram of segments (header/control/data)
- field table (name, type, offset, writer, reader)
- a version/hash definition
- an example of producing/consuming code paths

This book will eventually include pinned layouts in Appendix “Buffer Layouts (Draft).”

---

**Prev:** [Rollout Transport: Ring Buffer (N→1)](rollout-ring-buffer.md)  
**Next:** [Part IV — The Show Environment](../part-4-show-environment/README.md)


