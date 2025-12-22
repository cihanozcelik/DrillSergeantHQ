## CPU Fallback Strategy: Designing Graceful Degradation for Web Apps

### Who this is for
You’re building a high-performance web app that uses advanced capabilities (GPU compute, WebGPU rendering, heavy parallelism). You want the app to keep working—even if the user’s device doesn’t support those features.

This article explains how to design a **CPU fallback strategy** that is technically sound and product-friendly.

### What you’ll learn
- What “graceful degradation” actually means
- How to detect missing features reliably
- Common fallback designs for rendering and compute
- How to set user expectations (performance and quality)
- A practical checklist for shipping fallbacks

---

## 1) What is graceful degradation?

Graceful degradation means:

- the app still functions when a feature is unavailable
- quality or performance may decrease, but the user can still achieve goals

In contrast, a hard failure shows:

- a blank screen
- a cryptic error
- “not supported” with no next steps

For serious products, graceful degradation is part of correctness.

---

## 2) The two classes of “fallback”

### A) Capability fallback (different implementation)
Example:
- GPU compute → CPU compute
- WebGPU renderer → WebGL or Canvas2D renderer

### B) Quality fallback (same implementation, reduced workload)
Example:
- lower resolution
- fewer particles/agents
- smaller batch sizes
- fewer simulation steps per second

Often you’ll use both:
- switch to CPU path
- reduce workload so CPU path remains usable

---

## 3) Feature detection (how to decide what path to run)

Never rely on user-agent sniffing. Detect capabilities directly.

Examples:

- GPU availability (conceptually):
  - “is the API present?”
  - “can I request an adapter/device?”
- Off-main-thread rendering:
  - “is OffscreenCanvas supported?”
  - “can I create the context I need?”
- Shared memory / threads:
  - “is cross-origin isolation active?”

Practical rule:
> Detect “works” by attempting initialization and handling failures cleanly.

---

## 4) Designing a CPU fallback for compute-heavy features

CPU fallback is often 5–50× slower than GPU for parallel workloads. You need a plan:

### Strategy A: smaller workloads
- fewer environments / agents
- smaller model sizes
- lower simulation tick rate

### Strategy B: parallelize CPU work (when available)
- use worker pools
- use SIMD-friendly layouts (SoA)
- batch operations to reduce overhead

### Strategy C: progressive features
- “basic mode” first, then “accelerated mode” when available

This avoids punishing users with unsupported devices.

---

## 5) Rendering fallbacks (the common ladder)

A practical approach is a fallback ladder:

1) preferred renderer (highest quality/performance)
2) mid-tier renderer (widely supported)
3) simplest renderer (almost universal)

Example ladder:
- WebGPU → WebGL → Canvas2D

Key idea:
- Keep the scene/model consistent across renderers (same state, different drawing backends).

---

## 6) UX matters: don’t hide the truth

Users notice performance differences. Good fallback UX includes:

- a visible “compatibility mode” label
- an explanation of what’s reduced (e.g., “lower FPS, fewer agents”)
- a link to requirements and how to upgrade (browser settings, GPU flags, etc.)

Avoid:
- silently running a CPU path at 2 FPS with no explanation

---

## 7) Observability: measure the fallback path too

Log and/or display:
- frame time / FPS
- compute time per step
- steps per second
- whether GPU/accelerated mode is active

This helps users understand the experience and helps you debug real-world devices.

---

## 8) Shipping checklist

- **Detection**
  - capability detection is reliable (no UA sniffing)
  - initialization failures are handled cleanly
- **Fallback ladder**
  - at least one widely-supported rendering mode exists
  - at least one “functional” compute mode exists
- **Performance**
  - CPU mode has a reduced workload configuration
  - hard caps prevent runaway CPU usage
- **UX**
  - user sees which mode they’re in and why
  - clear upgrade guidance exists

