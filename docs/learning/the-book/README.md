# DrillSergeantHQ: The Architecture of Real‑Time Browser RL

This folder contains **the book**: a long-form, systems-level explanation of DrillSergeantHQ, written in the style of an engine/renderer architecture text—focused on **boundaries**, **data flow**, **performance constraints**, and **debuggability**.

DrillSergeantHQ is a browser-only agent training playground where you can watch a live “show match” while background training continuously improves the policy—then **hot‑swap** those weights into the running match without restarting.

## Book index (Table of Contents)

- [Front Matter](front-matter/README.md)
  - [Title Page](front-matter/title-page.md)
  - [Preface](front-matter/preface.md)
  - [How to Use This Book](front-matter/how-to-use-this-book.md)
  - [Notation and Conventions](front-matter/notation.md)

- [Part I — The Product and the Constraints](part-1-product-and-constraints/README.md)
  - [What DrillSergeantHQ Is](part-1-product-and-constraints/what-is-drillsergeanthq.md)
  - [The “Live Improvement” Promise](part-1-product-and-constraints/live-improvement-promise.md)
  - [Goals, Non-Goals, and Dependency Policy](part-1-product-and-constraints/goals-non-goals-dependency-policy.md)
  - [Browser Constraints That Lock the Design](part-1-product-and-constraints/browser-constraints.md)

- [Part II — System Architecture](part-2-system-architecture/README.md)
  - [The “WTF Map”: One Tick Through the Whole System](part-2-system-architecture/wtf-map.md)
  - [Worker Topology and Responsibilities](part-2-system-architecture/worker-topology.md)
  - [Timing and Loops: Show vs Training](part-2-system-architecture/timing-and-loops.md)
  - [Protocols: Messages and Contracts](part-2-system-architecture/protocols.md)
  - [Shared Memory Strategy (SAB First)](part-2-system-architecture/shared-memory-strategy.md)

- [Part III — Shared Memory Contracts](part-3-shared-memory/README.md)
  - [Weights Publishing: Double Buffer + Atomics](part-3-shared-memory/weights-double-buffer.md)
  - [Rollout Transport: Ring Buffer (N→1)](part-3-shared-memory/rollout-ring-buffer.md)
  - [ABI Mindset: Layouts, Alignment, and Versioning](part-3-shared-memory/abi-and-versioning.md)

- [Part IV — The Show Environment](part-4-show-environment/README.md)
  - [Deterministic Fixed Timestep Simulation](part-4-show-environment/fixed-timestep.md)
  - [Action Selection and “No Teleport Jumps”](part-4-show-environment/action-selection.md)
  - [Hot-Swapping Policies Without Breaking Reality](part-4-show-environment/hot-swapping.md)

- [Part V — Training System](part-5-training-system/README.md)
  - [Pipeline Overview: Rollout → GAE → PPO → Publish](part-5-training-system/pipeline-overview.md)
  - [PPO Objective (Clipped Surrogate)](part-5-training-system/ppo-objective.md)
  - [GAE and Returns](part-5-training-system/gae.md)
  - [Rewards and Shaping: Learnability Without Cheating](part-5-training-system/rewards-and-shaping.md)
  - [Self-Play and Stabilization](part-5-training-system/self-play.md)
  - [Telemetry: Making Training Observable](part-5-training-system/telemetry.md)

- [Part VI — GPU Compute](part-6-gpu-compute/README.md)
  - [Where WebGPU Is Used](part-6-gpu-compute/where-webgpu-is-used.md)
  - [Kernel Inventory for a PPO Learner](part-6-gpu-compute/kernel-inventory.md)
  - [Validation Mode: CPU Reference Paths](part-6-gpu-compute/validation-mode.md)

- [Part VII — Storage and Deployment](part-7-storage-and-deployment/README.md)
  - [Persistence with OPFS](part-7-storage-and-deployment/opfs.md)
  - [COOP/COEP and Cross-Origin Isolation](part-7-storage-and-deployment/cross-origin-isolation.md)

- [Appendices](appendices/README.md)
  - [Glossary](appendices/glossary.md)
  - [System Diagrams (Text + Mermaid)](appendices/diagrams.md)
  - [Message Schemas (Draft)](appendices/message-schemas.md)
  - [Buffer Layouts (Draft)](appendices/buffer-layouts.md)
  - [Math Cheat Sheet](appendices/math-cheat-sheet.md)
  - [Roadmap (Phases)](appendices/roadmap.md)

- [Author Notes (internal)](_author/README.md)
  - [Book Outline and Writing Checklist](_author/outline.md)

## How to read

- **Read linearly (recommended)**: click **Next** at the bottom of each chapter.
- **Jump by topic**:
  - **Architecture glue**: Part II (especially the WTF Map)
  - **Implementation contracts**: Part III + “Message Schemas” / “Buffer Layouts”
  - **Performance/debug**: Timing & Loops, GPU Compute, Validation Mode

## Scope and philosophy

- This is **not** a generic PPO textbook, and it is **not** a generic WebGPU tutorial.
- We treat DrillSergeantHQ as a system: **product promise → constraints → architecture → contracts → loops → performance/validation**.
- We assume the project’s “core value” is built in-house: **simulation, RL math, training loop, memory layouts, kernels**.

## Where the canonical design comes from

The authoritative spec that seeded this book lives at:

- `docs/technical-design.md`

This book turns that spec into a cohesive narrative, adds rationale, adds “why,” and introduces the kinds of invariants and contracts that make the system scalable for a team.

---

**Next:** [Front Matter](front-matter/README.md)


