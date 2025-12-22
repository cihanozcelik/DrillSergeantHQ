# How to Use This Book

This is a **systems book**. You can read it linearly, but you’ll get more value if you treat it like a reference manual with a storyline.

## Reading paths

### I want to understand the architecture

Read:

- Part I (product + constraints)
- Part II (worker topology, loops, protocols)
- Part III (SAB contracts)

### I want to implement the system

Read:

- Part II: “Protocols” + “Shared Memory Strategy”
- Part III: “Weights Double Buffer”, “Rollout Ring Buffer”, “ABI and Versioning”
- Appendices: message schemas and buffer layouts

### I want to debug or optimize

Read:

- Part II: “Timing and Loops”
- Part V: “Telemetry”
- Part VI: “Validation Mode”

## What we deliberately repeat

Some concepts show up again and again:

- **Contracts beat cleverness**: shared layouts and message schemas are first-class.
- **Determinism is a feature**: if you can’t reproduce it, you can’t optimize it.
- **Copy-free beats pretty**: if it’s hot, it should be SoA and shared.
- **The show match is sacred**: it must stay responsive and visually stable.

When you see repetition, it’s usually because the system depends on that invariant in multiple places.

---

**Prev:** [Preface](preface.md)  
**Next:** [Notation and Conventions](notation.md)


