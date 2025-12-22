# Part V — Training System

Part IV explained how we keep the show match stable while policies change.

Now we explain the other half of the promise: how the system produces better policies continuously—fast enough to *feel* live, but stable enough to avoid constant collapse.

DrillSergeantHQ uses a deliberately “workhorse” algorithmic core:

- **PPO** (clipped surrogate objective)
- **GAE** (advantage estimation)
- optional **self-play stabilization** (opponent pool)

This part is written like an engine subsystem:

- start with the pipeline (what happens in what order)
- define the invariants (what must stay true)
- describe the failure modes (how training breaks, and how we detect it)

---

**Prev:** [Hot-Swapping Policies Without Breaking Reality](../part-4-show-environment/hot-swapping.md)  
**Next:** [Pipeline Overview: Rollout → GAE → PPO → Publish](pipeline-overview.md)


