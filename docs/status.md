# Project Status

This repository is currently in a **documentation-first (design/learning) phase**.

## What exists today

- ✅ Architecture and constraints are documented (`docs/technical-design.md`)
- ✅ Long-form learning book and appendices (`docs/learning/the-book/`)
- ✅ Tutorials by topic (`docs/learning/tutorials-by-topic/`)
- ✅ Roadmap with definitions of done (`docs/learning/the-book/appendices/roadmap.md`)

## What does *not* exist yet

- ❌ A runnable web app (`/web`)
- ❌ A Rust workspace (`/rust`)
- ❌ Determinism/perf test harnesses
- ❌ Release process / versioned artifacts

## Near-term intent (documentation milestones)

- Clarify **normative vs explanatory** docs:
  - “Normative” = contracts that implementations must follow (ABI/layouts/protocols)
  - “Explanatory” = teaching material and rationale
- Consolidate any future hard contracts into a dedicated place once implementation begins.

## Implementation phases (planned)

These phases are described in the technical design and the roadmap appendix.

- **Phase A — Foundation**: COOP/COEP + workers + WASM threads
- **Phase B — CPU sim + show rendering**
- **Phase C — Rollout engine + RL data structures**
- **Phase D — PPO update (GPU compute)**
- **Phase E — Show hot-swap + “live improvement” UX**
- **Phase F — Self-play pool + stabilization**
- **Phase G — GPU simulation backend (optional)**


