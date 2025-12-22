# Part VI — GPU Compute

DrillSergeantHQ uses the GPU for two different jobs:

- **rendering** the show match
- **accelerating training compute** (forward/backward, reductions, optimizer updates)

This part is not a WebGPU tutorial. It is an architecture explanation:

- where GPU compute sits in the system
- what kernels we need for a PPO learner
- how we keep the GPU path honest (validation mode)

---

**Prev:** [Telemetry: Making Training Observable](../part-5-training-system/telemetry.md)  
**Next:** [Where WebGPU Is Used](where-webgpu-is-used.md)


