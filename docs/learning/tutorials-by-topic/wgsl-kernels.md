## WGSL Compute Kernels: How to Think About GPU Compute in WebGPU

### Who this is for
You’ve heard “write compute kernels in WGSL” and you want to understand what that means. This tutorial is a standalone introduction to GPU compute with practical optimization principles—without assuming a specific application.

### What you’ll learn
- What a **compute kernel** is
- How to map work to GPU threads (workgroups, invocations)
- The most important performance concepts (memory bandwidth, coalescing, occupancy)
- Common WGSL patterns (bounds checks, indexing, reductions)
- A debugging and optimization checklist

---

## 1) What is a compute kernel?

A **compute kernel** is a function executed many times in parallel on the GPU. You define:

- how many “invocations” run
- which data each invocation reads/writes

GPU compute is powerful when:
- you can do the same operation across many elements
- memory access is predictable
- branching is limited

---

## 2) Core WGSL concepts (quick glossary)

- **Invocation**: one logical thread running your shader code.
- **Workgroup**: a small group of invocations that can share fast local memory.
- **Global invocation id**: unique index for the invocation in the whole dispatch.
- **Workgroup size**: how many invocations are in a workgroup (e.g., 64, 128, 256).
- **Storage buffer**: GPU buffer for large read/write arrays.

---

## 3) Mapping work: from arrays to threads

The most common pattern is “one invocation per element.”

Example mental model:
- you have an array `X` of length `N`
- each invocation handles one index `i`

Your kernel:
- computes `i` from `global_invocation_id`
- checks `i < N` (bounds check)
- reads `X[i]`, writes `Y[i]`

Bounds checks are crucial because dispatch sizes are often rounded up.

---

## 4) Performance principle #1: memory is the bottleneck

Many compute kernels are limited by memory bandwidth, not arithmetic.

To optimize:
- minimize global memory reads/writes
- reuse data via workgroup local memory when possible
- keep accesses contiguous (coalesced)

---

## 5) Performance principle #2: avoid divergent branches

If neighboring invocations take different branches, the GPU may serialize work.

Guidelines:
- keep branches uniform across threads in a workgroup
- use predication (compute and multiply by mask) when appropriate

---

## 6) Common kernel patterns

### A) Elementwise operations
Examples: add, multiply, activation functions.

Usually straightforward and fast if memory is contiguous.

### B) Reductions (sum/mean/max)
Harder because many threads must combine results.

Typical approach:
- partial sums per workgroup
- write partial results
- run a second pass to reduce partials

### C) Matrix multiplication-like workloads
Performance depends heavily on tiling and memory reuse:
- load tiles into workgroup memory
- compute multiple outputs per invocation if it helps reuse

---

## 7) Debugging WGSL kernels

GPU debugging is harder than CPU debugging. Practical techniques:

- start with a CPU reference implementation and compare outputs
- run small sizes (N=32, 64) where you can inspect results
- add “debug buffers” that store intermediate values for a small subset of invocations
- check for NaNs/Infs early

---

## 8) A practical optimization checklist

- **Correctness first**
  - CPU reference comparison with tolerances
  - bounds checks everywhere
- **Data layout**
  - prefer SoA-style contiguous arrays
  - align buffers sensibly (often 16-byte multiples help)
- **Dispatch**
  - choose a reasonable workgroup size (start with 64 or 128)
  - ensure enough work to saturate GPU
- **Memory**
  - minimize global reads/writes
  - use workgroup memory for reuse
- **Control flow**
  - reduce divergent branching

