# Validation Mode: CPU Reference Paths

GPU kernels are fast—and unforgiving.

A single indexing bug can produce a model that “trains” while quietly learning nonsense. That’s unacceptable for a project that aims to teach fundamentals.

So DrillSergeantHQ adopts an engine-like stance:

> Every GPU kernel should have a CPU reference implementation.

## What validation mode is

Validation mode runs occasional cross-checks:

- compute a result on CPU (reference path)
- compute a result on GPU
- compare within a tolerance
- if mismatch: log a warning (or fail in dev builds)

You don’t validate every step (too slow). You validate enough to catch regressions early.

## What to validate first

Prioritize kernels that can silently corrupt learning:

- reductions (mean/std, loss sums)
- logprob/entropy math
- optimizer updates (Adam)
- any kernel with tricky indexing/strides

## Tolerance and determinism

Floating point math differs across GPU/CPU.

Validation mode should:

- use relative/absolute tolerance
- compare aggregates as well as element-wise samples
- record the seed and batch that triggered mismatch

## Why this is part of the *product*

DrillSergeantHQ is a learning playground. If the GPU path is a black box, the project becomes “trust the GPU.”

Validation mode keeps the system honest and makes performance work safe to iterate on.

---

**Prev:** [Kernel Inventory for a PPO Learner](kernel-inventory.md)  
**Next:** [Part VII — Storage and Deployment](../part-7-storage-and-deployment/README.md)


