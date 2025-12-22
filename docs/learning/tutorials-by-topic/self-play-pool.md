## Self‑Play Pools: How to Build a “League” of Opponents That Makes Learning Stable

### Who this is for
You’ve accepted that self-play needs stabilization, and you’ve heard “use an opponent pool.” This tutorial is about the practical design: what goes into the pool, how you sample from it, and how you keep it from turning into a junk drawer.

### What you’ll learn
- What an opponent pool is and why it helps
- Snapshot strategies (when to save, what to save)
- Sampling strategies (uniform, recency bias, Elo-based)
- How to prevent forgetting and cycling
- A concrete pool design you can implement

---

## 1) What is an opponent pool?

An **opponent pool** is a collection of past policies (snapshots) that you train against.

Instead of always fighting the latest version of yourself, you fight a **mixture**:

- some games vs the latest opponent
- some games vs older snapshots
- optionally some games vs scripted baselines

This makes training data more diverse and reduces non-stationarity.

---

## 2) Why “latest-only” training is fragile

Training only against the latest opponent can cause:

- overfitting to the current meta
- strategy cycling (rock-paper-scissors loops)
- catastrophic forgetting (agent forgets how to beat older styles)

An opponent pool works like regression testing:
- “Are we still good against what used to beat us?”

---

## 3) What to store in a pool snapshot

At minimum:
- model weights (policy + value if applicable)
- a version id

Highly recommended metadata:
- training step / timestamp
- hyperparameters (lr, gamma, lambda, etc.)
- evaluation stats at snapshot time
- code version / commit hash (if in a repo)

Without metadata, you won’t know why a snapshot is strong or weak.

---

## 4) When to add snapshots

Common snapshot schedules:

- **every N updates** (simple, consistent)
- **when performance improves** (adaptive)
- **time-based** (every X minutes)

Practical guidance:
- start with every N updates (e.g., every 50–200)
- then adjust once you understand how quickly strategies change

---

## 5) Sampling strategies (how to pick opponents)

### A) Uniform sampling
Pick a random snapshot from the pool.

Pros: simple, broad coverage  
Cons: wastes time on very weak opponents late in training

### B) Recency-biased sampling
Sample newer snapshots more often.

Pros: focuses on current meta  
Cons: can reintroduce cycling and forgetting

### C) Elo-based sampling (league training style)
Maintain ratings for snapshots and sample based on:
- closeness in rating (fair matches)
- or difficulty ramp (slightly stronger opponents)

Pros: targeted learning, better curriculum  
Cons: needs more evaluation infrastructure

### D) Mixture recipe (a good default)
Use a mixture like:

- 50% latest
- 40% pool uniform
- 10% hard opponents (top-rated or known “counters”)

This is simple and surprisingly effective.

---

## 6) Keeping the pool healthy (garbage in, garbage out)

Pools can degrade if:
- you store too many near-identical snapshots
- you keep snapshots that are clearly broken
- you never prune old data

Pruning strategies:
- keep a rolling window (last K snapshots)
- keep “milestone” snapshots (best-ever, diverse set)
- keep top-rated snapshots plus a random subset for diversity

---

## 7) Worked design: a small self-play league

Here’s a concrete pool design you can implement:

- Pool size: 100 snapshots
- Snapshot every 100 updates
- Keep:
  - last 60 snapshots (recency)
  - best 20 snapshots by evaluation rating
  - 20 random older snapshots (diversity)

Sampling each episode:

- 50%: latest
- 30%: uniform from pool
- 20%: “hard set” (top-rated or recent counters)

Track:
- win-rate against a fixed evaluation set
- Elo ratings updated from match results

---

## 8) Checklist

- Pool snapshots include metadata (step, config)
- Sampling includes both latest and diverse opponents
- You evaluate against a fixed set to detect regressions
- You prune the pool so it stays diverse and useful
- You can reproduce and compare runs across seeds

