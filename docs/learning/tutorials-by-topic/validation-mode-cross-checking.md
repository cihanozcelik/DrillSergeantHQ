## Validation Mode and Cross‑Checking: How to Debug Fast Systems Without Guessing

### Who this is for
You’re building something performance-heavy (GPU compute, SIMD, multithreaded code, custom kernels) and you keep hitting the worst kind of bug: **the output is wrong but nothing crashes**.

The cure is a discipline: a “validation mode” that compares a fast implementation against a trusted reference and tells you *when* and *how* they diverge.

### What you’ll learn
- What “validation mode” means in engineering terms
- The **reference implementation** pattern (CPU vs GPU, slow vs fast)
- How to compare outputs safely (tolerances, checksums)
- How to localize divergence (binary search over steps)
- A checklist you can adopt immediately

---

## 1) Why validation mode exists

Fast code is often:
- parallel
- vectorized
- numerically sensitive
- hardware-dependent

That makes debugging by “print statements” ineffective.

Validation mode provides:
- a trusted baseline result
- a way to detect divergence early
- structured evidence for fixing bugs

---

## 2) Core concept: the reference path

You implement the same logic twice:

- **Reference path**: simple, readable, correct (even if slow)
- **Fast path**: optimized version (GPU kernel, SIMD, multithreaded)

Then you run both on the same inputs and compare.

This pattern is used everywhere:
- crypto (constant-time vs straightforward reference)
- compilers (optimized vs unoptimized evaluation)
- physics sims (scalar vs SIMD)
- GPU kernels (CPU reference vs shader compute)

---

## 3) What to compare (and how)

### Exact vs approximate equality

For integers:
- expect exact matches

For floats:
- expect small differences due to rounding and operation ordering

Use tolerances:
- absolute tolerance: $|a-b| \le \epsilon$
- relative tolerance: $|a-b| \le \epsilon \cdot \max(1, |a|, |b|)$

Practical suggestion:
- use both (abs + relative)
- log the maximum error and where it occurred

---

## 4) Localization strategy: “find the first bad step”

When systems diverge after many steps, you need to find the earliest point of mismatch.

### Technique: checkpoint + replay

1) Run both implementations in lockstep.
2) Every K steps, store a compact snapshot (or a hash) of state.
3) When mismatch occurs, binary search:
   - replay from the last matching checkpoint
   - halve the interval until you find the first mismatching step

This turns a “somewhere in 10 million steps” problem into “step 12,345 is wrong.”

---

## 5) Invariants: cheap correctness tests

Invariants are properties that should always hold.

Examples:
- probabilities sum to ~1
- no NaNs/Infs in tensors
- conserved quantities (within tolerance) in physics
- indices stay within bounds

Run invariant checks in validation mode and fail fast when violated.

---

## 6) Instrumentation that actually helps

Useful logging in validation mode:
- first mismatch index and field name
- maximum absolute/relative error
- input seeds and configuration
- a small slice of the tensor around the mismatch

Avoid:
- dumping entire tensors every step (too slow, too noisy)

---

## 7) Debugging “black box” ML behavior

Training systems can hide bugs because the model will “adapt” to errors.

Signs you need validation mode:
- training sometimes “works” but isn’t reproducible
- value loss explodes on some seeds
- outputs become NaN occasionally
- GPU and CPU results differ dramatically

Validation mode lets you separate:
- **math bugs** (wrong kernel)
- **data bugs** (wrong layout/stride)
- **numerical issues** (underflow/overflow)

---

## 8) A practical validation-mode checklist

- Build a **reference implementation** you trust.
- Ensure both paths share identical inputs (seeded RNG, same layout).
- Compare with appropriate tolerances (float-safe).
- Detect the **first divergence step** (checkpoint + replay).
- Add invariants (no NaNs, sum checks, bounds checks).
- Make it easy to enable/disable (compile flag or runtime option).

