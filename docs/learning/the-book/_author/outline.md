# Book Outline and Writing Checklist

This is an internal checklist to keep quality consistent as the book grows.

## Chapter template (keep this structure)

1. **Purpose**: what question the chapter answers
2. **Invariants**: what must remain true
3. **Loop**: where this fits in runtime timing
4. **Contract**: schemas/layouts/ownership
5. **Failure modes**: how it breaks and how we observe it

## Style rules

- Prefer “systems language” over hype.
- Use short paragraphs and explicit lists; avoid hand-wavy prose.
- Reuse the same diagrams repeatedly to anchor the reader.
- Treat protocols and buffer layouts as first-class artifacts.

## Open questions to resolve as code lands

- definitive observation vector schema (dimensions, normalization)
- definitive action encoding table and mapping to physics controls
- rollout chunk layout (offsets/strides) for SAB ring buffers
- kernel naming and buffer binding layout conventions
- fallback behavior when WebGPU is unavailable (CPU training path)

## Quality checklist per chapter

- Does it connect back to the **WTF map**?
- Does it state ownership (which worker/subsystem)?
- Does it name the performance budget (FPS/SPS/UPS) impacted?
- Does it define a contract (even if draft)?
- Does it list at least 2 real failure modes and observability signals?

---

**Prev:** [Author Notes (internal)](README.md)


