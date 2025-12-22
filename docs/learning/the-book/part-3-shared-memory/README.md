# Part III — Shared Memory Contracts

Part II tells you *why* the system is shaped the way it is.

Part III tells you *exactly what bytes exist* and *what they mean*.

If DrillSergeantHQ is an engine, shared memory contracts are its header files:

- they define what’s allowed across worker boundaries
- they make performance predictable
- they prevent “it works on my machine” bugs

This part covers two contracts:

1. **Weights publishing** (double buffer + atomics)
2. **Rollout transport** (ring buffer, N producers → 1 consumer)

And one mindset:

3. **ABI discipline** (alignment, versioning, and failure behavior)

---

**Prev:** [Shared Memory Strategy (SAB First)](../part-2-system-architecture/shared-memory-strategy.md)  
**Next:** [Weights Publishing: Double Buffer + Atomics](weights-double-buffer.md)


