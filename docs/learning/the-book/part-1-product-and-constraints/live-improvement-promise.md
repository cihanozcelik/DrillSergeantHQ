# The “Live Improvement” Promise

If this were a traditional engine book, we’d talk about “60 FPS” as a contract. DrillSergeantHQ has a similar contract, but it’s psychological:

> Training must feel continuous, observable, and responsive.

This breaks down into four measurable requirements.

## 1) The show match must stay responsive

The show match is where the user’s trust is won or lost. It cannot stutter because training is doing “something important.” The show match therefore runs in its own worker with its own loop budget.

## 2) The learner must publish improvements frequently

A learner that updates every 5 minutes is technically learning, but it will feel dead. For the user to *feel* learning:

- checkpoints must publish at human-noticeable cadence (often ~1 Hz is enough)
- the show match must actually adopt those checkpoints (hot‑swap)

## 3) The system must support “normal controls”

Users expect:

- play / pause
- speed control
- reset
- deterministic replay (for debugging)

These controls conflict with naive training systems, so we explicitly design loops and contracts to support them.

## 4) User interaction must not poison training

The show match is both a demo and a test harness. But the training dataset must remain well-defined. In v1:

- user interventions are treated as evaluation
- rollouts for learning come from separate training environments

This split simplifies learning stability and makes results interpretable.

## The design consequence

Once you accept these four requirements, the architecture is effectively forced:

- parallel runtimes (workers)
- explicit messaging for controls
- shared memory for high-throughput data (weights, rollouts)
- deterministic simulation and validation paths to preserve trust

The rest of this book is the details of making that architecture real.

---

**Prev:** [What DrillSergeantHQ Is](what-is-drillsergeanthq.md)  
**Next:** [Goals, Non-Goals, and Dependency Policy](goals-non-goals-dependency-policy.md)


