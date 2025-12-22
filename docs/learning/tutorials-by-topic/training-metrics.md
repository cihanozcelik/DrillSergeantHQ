## Training Metrics and Telemetry: How to Know Your Model Is Actually Learning

### Who this is for
You’re training a model (often RL, sometimes supervised) and you want to avoid the most common trap: **watching one number go down and assuming everything is fine**. This tutorial shows which metrics matter, what they mean, and how to interpret them.

### What you’ll learn
- The difference between **loss metrics** and **outcome metrics**
- What to log in reinforcement learning vs supervised learning
- Throughput metrics (SPS, updates/sec) and why they matter
- Early warning signs of collapse or bugs
- A practical “minimum viable dashboard”

---

## 1) Why metrics are not optional

Training is a dynamic system. Without telemetry:

- you won’t notice when learning stalls
- you won’t catch reward hacking
- you won’t know whether a change helped or hurt

A good metrics setup turns “mystery” into “diagnosis.”

---

## 2) Two categories: optimization vs outcomes

### Optimization metrics (internal health)
These tell you whether the learning algorithm is behaving:

- policy loss / surrogate objective
- value loss
- entropy
- KL divergence (policy shift)
- gradient norms

They are necessary but not sufficient.

### Outcome metrics (what you actually care about)
These tell you whether the agent/model is improving in the real sense:

- win rate / success rate
- average return
- episode length (sometimes good, sometimes bad)
- task-specific score

Outcome metrics are your “product truth.”

---

## 3) Core RL metrics (what they mean)

### Average episode return
Often reported as `avg_reward` or `avg_return`.

- Rising return usually indicates learning.
- But return can rise from reward hacking—pair it with success metrics.

### Success / win rate
If your task has a clear success condition, log it.

Pro tip: use an **EMA** (exponential moving average) to reduce noise:

- `win_rate_ema`

### Episode length
Interpret with context:

- In “reach goal quickly,” shorter is better.
- In “survive,” longer is better.
- Sudden jumps often indicate a behavior change (good or bad).

---

## 4) PPO-specific health metrics

### Policy loss / surrogate objective
This is a training objective, not a score. Watch for:

- exploding values (instability)
- flatlining (no learning signal)

### Value loss
If value loss is huge or increasing, the critic may be failing:

- poor observations
- too high learning rate
- advantage/return computation bug

### Entropy
Entropy measures how random the policy is.

- Early training: entropy should be relatively high (exploration).
- Later: entropy tends to drop as the policy becomes confident.

If entropy collapses too early, the policy may be prematurely deterministic.

### Approximate KL divergence
KL tells you how much the policy is changing per update.

- Too high: updates are too aggressive (reduce lr, reduce epochs, tighten clip)
- Too low: updates are tiny (increase lr, loosen clip, check advantages)

### Clip fraction
What fraction of samples hit PPO clipping.

- Very high clip fraction can indicate overly large updates or too-small clip range.

---

## 5) Performance metrics (the “are we wasting time?” numbers)

### Steps per second (SPS)
How fast you collect environment steps.

If SPS drops:
- bottleneck in simulation, inference, or communication
- logging overhead too high

### Updates per second
How quickly the learner runs optimization steps.

If updates/sec is low:
- GPU underutilized (batch size too small)
- data pipeline feeding learner too slowly

---

## 6) Hyperparameter tracking (for reproducibility)

Log your “current config” every run:

- learning rate (`current_lr`)
- PPO clip (`clip_epsilon`)
- batch sizes, epochs, entropy coefficient, value coefficient
- gamma, lambda
- random seeds

Without this, you can’t interpret changes or reproduce results.

---

## 7) Early warning signs (read this when something feels off)

- **Return goes up, win rate stays flat**
  - likely reward hacking or metric mismatch
- **Entropy collapses quickly**
  - policy became deterministic too early
- **Value loss explodes**
  - critic instability or return computation bug
- **KL spikes**
  - learning rate too high or too many epochs per batch
- **SPS suddenly drops**
  - pipeline regression (logging, contention, memory pressure)

---

## 8) Minimum viable dashboard (start here)

If you only log a few graphs, log these:

- outcome: success/win rate, average return
- health: entropy, approx KL, value loss
- performance: steps per second, updates per second

Add domain-specific metrics next (e.g., distance-to-goal, collision rate).

