# Where WebGPU Is Used

WebGPU is not “the point.” It’s an enabling layer that lets the system hit throughput targets on consumer hardware.

DrillSergeantHQ uses WebGPU in two places:

## 1) Render/Eval worker: rendering the show match

Rendering is time-critical:

- stable frame times matter more than peak throughput
- spikes are visible and break trust

So the Render/Eval worker owns its render pipeline and keeps it isolated from training.

## 2) Trainer worker: training compute

Training compute is throughput-oriented:

- large-batch forward passes
- reductions (means/variances/entropy)
- backward passes
- Adam updates

WebGPU is well-suited for this because:

- it can run in workers
- it supports compute pipelines
- it aligns with SoA buffer layouts

## Why not “GPU for everything” in v1?

The technical design explicitly calls GPU simulation an optional phase because:

- collision/impulse logic is complex to debug in WGSL
- correctness matters more than theoretical throughput early on

The v1 path is:

- CPU rollouts (fast, debuggable)
- GPU learner (big win, contained complexity)

Then later:

- optional GPU rollout backend once CPU path is stable and validated

---

**Prev:** [Part VI — GPU Compute](README.md)  
**Next:** [Kernel Inventory for a PPO Learner](kernel-inventory.md)


