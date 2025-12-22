## WASM Threads and Rayon: Parallelism in WebAssembly (Standalone Guide)

### Who this is for
You’re writing performance-critical code in Rust and compiling it to WebAssembly. You want to use multiple CPU cores in the browser (real parallelism), and you’ve heard that Rayon can work in WASM—sometimes.

This tutorial explains the moving pieces and the mental model, without assuming any specific project.

### What you’ll learn
- What “WASM threads” means
- Why the web needs **SharedArrayBuffer** for threading
- How Rayon’s thread pool maps to Web Workers in the browser
- The setup requirements (and why they exist)
- A practical checklist and common pitfalls

---

## 1) What are “WASM threads”?

WebAssembly can support threads via:

- **shared linear memory** (shared memory between threads)
- atomic operations

On the web, shared memory is typically provided via `SharedArrayBuffer`.

Threads in WASM aren’t OS threads in the classic sense; they are commonly backed by Web Workers that execute WASM code with shared memory.

---

## 2) Why SharedArrayBuffer is required

Threads need shared memory so multiple workers can read/write the same memory region.

Without shared memory:
- each worker would have its own separate memory
- copying would be required for communication
- many parallel patterns would become slow or impossible

Because shared memory can enable security side-channels, browsers often require cross-origin isolation for `SharedArrayBuffer`.

---

## 3) How Rayon works (and why it’s attractive)

Rayon is a Rust data-parallelism library that lets you write:

- `par_iter()`
- parallel reductions
- parallel sorts

…without manually managing threads.

On native platforms, Rayon uses a thread pool of OS threads.
On the web, you don’t get OS threads directly; you typically use Web Workers.

So “Rayon in WASM” means:
- create a pool of workers
- start a Rayon thread pool that dispatches work onto those workers

---

## 4) The high-level setup (what has to happen)

A functioning setup usually needs:

- Browser support for `SharedArrayBuffer`
- Cross-origin isolation headers (COOP/COEP) in many environments
- A worker pool initialized early in app startup
- WASM built with thread support enabled

If any of those are missing, Rayon will either:
- run single-threaded
- fail to initialize
- or crash/behave unpredictably

---

## 5) Common pitfalls

- **`SharedArrayBuffer` unavailable**
  - cause: missing COOP/COEP headers or incompatible embeds
- **Thread pool not initialized**
  - cause: you never created workers or called init logic
- **Too many workers**
  - cause: creating more workers than CPU cores can hurt performance
- **Heavy work on the main thread**
  - even with Rayon, if you run blocking work on the main thread you will still get UI jank

---

## 6) Practical guidance

- Start with a small worker count:
  - `cores - 1` is often a safe first guess (leave room for UI)
- Measure throughput (steps/sec, time per batch).
- Build a clean fallback path:
  - if threading is unavailable, run single-threaded and reduce workload

---

## 7) Checklist

- `SharedArrayBuffer` is available where you run
- Cross-origin isolation is configured (if required)
- Worker pool initializes successfully
- WASM build enables atomics/threads and shared memory
- Rayon parallel loops are actually parallel (measure speedup)
- You have a fallback mode for unsupported environments

