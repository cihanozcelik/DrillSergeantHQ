# Self-Play and Stabilization

In competitive environments, “the environment” includes the opponent.

If you train against a fixed weak opponent, you overfit. If you train only against the latest opponent, you can oscillate: today’s policy beats yesterday’s, then tomorrow’s loses to last week’s.

Self-play is how we make improvement meaningful.

## The core idea

Both sides are controlled by policies drawn from some distribution:

- sometimes the latest policy
- sometimes older snapshots

This produces a curriculum without hand-authoring levels.

## Opponent pool (snapshot sampling)

The project’s technical design suggests an opponent pool:

- keep a set of snapshots (weights versions)
- sample opponents from a mixture

Why this works:

- it prevents overfitting to a single opponent
- it reduces “catastrophic forgetting”
- it dampens oscillations

## What the show match should do

The show match has a different job than training:

- it should demonstrate the latest policy’s behavior
- it should still be stable and interpretable

Common show modes:

- AI vs AI: latest vs pool
- Human vs AI: human vs latest

In v1, user behavior does not enter training data; it is evaluation.

## Metrics that matter

Self-play demands metrics beyond “avg reward”:

- **win-rate EMA** (against a reference distribution)
- optionally, ELO-like ratings across the snapshot pool

If you don’t measure relative performance, you can mistake “reward scale changes” for improvement.

## Failure modes

- **Policy cycling**: beats some opponents, loses to others in a loop  
  - response: broaden pool, adjust sampling distribution, add entropy

- **Over-conservatism**: pool too strong too early, learning stalls  
  - response: curriculum schedule (mix ratio) or staged opponents

Self-play is not a feature bolt-on. It is part of how “better” stays meaningful over time.

---

**Prev:** [Rewards and Shaping: Learnability Without Cheating](rewards-and-shaping.md)  
**Next:** [Telemetry: Making Training Observable](telemetry.md)


