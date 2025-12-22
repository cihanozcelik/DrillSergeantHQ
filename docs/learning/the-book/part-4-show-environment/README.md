# Part IV — The Show Environment

The show environment is the part of DrillSergeantHQ the user *trusts*.

Training can be noisy; learning curves can be weird; optimizers can explode. But the show match must remain:

- **responsive** (no stutter, no laggy controls)
- **continuous** (no teleporting, no “world resets” unless explicitly requested)
- **deterministic enough to debug** (repeatable seeds, repeatable stepping)

This part explains how to build a real-time simulation loop that can accept live policy hot‑swaps without breaking the illusion of physical continuity.

---

**Prev:** [ABI Mindset: Layouts, Alignment, and Versioning](../part-3-shared-memory/abi-and-versioning.md)  
**Next:** [Deterministic Fixed Timestep Simulation](fixed-timestep.md)


