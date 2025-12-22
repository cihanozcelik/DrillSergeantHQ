# Part VII — Storage and Deployment

Real systems don’t end at “it runs on my machine.”

DrillSergeantHQ lives inside a browser sandbox, which creates two practical requirements:

- **persistence**: saving/exporting/importing models locally
- **deployment**: shipping the right headers and constraints so SAB + WASM threads work

This part explains the system-level “plumbing” that makes the rest of the architecture possible.

---

**Prev:** [Validation Mode: CPU Reference Paths](../part-6-gpu-compute/validation-mode.md)  
**Next:** [Persistence with OPFS](opfs.md)


