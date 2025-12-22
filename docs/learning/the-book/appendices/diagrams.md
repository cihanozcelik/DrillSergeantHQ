# System Diagrams (Text + Mermaid)

This appendix exists so you can re-orient in 30 seconds.

## Worker topology (text)

```text
Main Thread (UI)  <---- small msgs ---->  Render/Eval Worker (show match)
        |
        | small msgs
        v
Trainer Worker  <---- SAB rollouts ----  Rollout Worker Pool (N)
   |
   +---- SAB weights (double buffer) ---> Render/Eval Worker
```

## WTF map (text)

```text
 UI → START_TRAINING → Trainer starts PPO loop
 Show loop continues → periodic weights hot-swap → visible improvement
 Rollouts stream (SAB ring) → Trainer consumes → updates → publishes (SAB double buffer)
```

## Mermaid topology (optional renderers)

```mermaid
flowchart LR
  UI[Main Thread<br/>UI] <-->|control msgs| SHOW[Render/Eval Worker<br/>Show env + Render]
  UI -->|control msgs| TR[Trainer Worker<br/>PPO + Publish]
  RW[Rollout Workers (N)<br/>Headless env batches] -->|SAB rollouts (ring)| TR
  TR -->|SAB weights (double buffer)| SHOW
```

---

**Prev:** [Glossary](glossary.md)  
**Next:** [Message Schemas (Draft)](message-schemas.md)


