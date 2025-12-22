## Training Schedulers: How to Control Learning Rate, Updates, and Data Flow Over Time

### Who this is for
You’re training a model and you’ve discovered the annoying truth: a single static configuration (“fixed learning rate, fixed batch size, fixed everything”) rarely gives the best results. You need **schedulers**—rules that change training behavior over time.

This article is standalone: it explains schedulers as a general concept across ML and RL.

### What you’ll learn
- What a “scheduler” is in training systems
- The big categories: **learning rate**, **data**, **optimization**, **exploration**
- Common schedules (warmup, cosine decay, step decay)
- A worked example schedule you can copy
- Failure modes and debugging tips

---

## 1) What is a training scheduler?

A **training scheduler** is any mechanism that changes training parameters as training progresses.

Examples:
- decrease learning rate after the model stabilizes
- increase batch size as training scales
- adjust exploration strength over time (common in RL)
- change how often you evaluate or checkpoint

Schedulers exist because training is non-stationary:

- early training needs big, exploratory moves
- later training needs small, precise refinements

---

## 2) The four scheduler types (a useful mental model)

### A) Learning-rate schedulers
Control the step size of optimization.

Common choices:
- warmup + constant
- warmup + cosine decay
- step decay (drop by factor at milestones)

### B) Data/throughput schedulers
Control *how much* and *what kind* of data you train on.

Examples:
- increase batch size over time
- curriculum learning: start easy, then increase difficulty
- change sampling distribution (hard negatives, rare events)

### C) Optimization schedulers
Control optimizer-side behavior.

Examples:
- gradient clipping thresholds
- weight decay schedules
- moving-average parameters (less common)

### D) Exploration schedulers (mostly RL)
Control how random or exploratory the policy is.

Examples:
- entropy coefficient decay
- epsilon-greedy schedule
- noise scale schedule (continuous control)

---

## 3) Learning rate schedules (the most important one)

### Warmup (why it exists)
Warmup means starting with a small learning rate and increasing it gradually for the first N steps.

Why:
- stabilizes training when gradients are initially chaotic
- avoids early divergence, especially with large batch sizes or adaptive optimizers

### Cosine decay (why people like it)
Cosine decay smoothly decreases learning rate toward a small value:

- fewer “sharp” transitions than step decay
- often works well with minimal tuning

### Step decay (simple and effective)
Drop learning rate at milestones, e.g., 1e-3 → 1e-4 → 1e-5.

Great when:
- you can identify plateaus
- you want predictable behavior

---

## 4) Worked example: warmup + cosine decay schedule

Let:
- total steps = 1,000,000
- warmup steps = 10,000
- base lr = 3e-4
- final lr = 3e-6

Pseudo-rule:

1) For step < warmup: linearly ramp 0 → base lr  
2) After warmup: cosine decay base lr → final lr

Implementation sketch:

```text
if step < warmup:
  lr = base_lr * (step / warmup)
else:
  p = (step - warmup) / (total - warmup)   // progress 0..1
  lr = final_lr + 0.5*(base_lr-final_lr)*(1 + cos(pi*p))
```

This is a solid default for many projects.

---

## 5) RL-specific: scheduling “update intensity”

In RL, you often have two clocks:

- **environment steps** (data generation)
- **gradient steps** (learning)

Schedulers can control:
- how many gradient steps per batch
- how large batches are (T × N)
- how often you refresh the policy in rollout workers

Common goal:
> keep the policy fresh enough (on-policy) while maximizing throughput.

---

## 6) What to log to tune schedulers

Schedulers are only useful if you can observe their effect.

Log:
- learning rate over time
- gradient norms and clip fraction (if PPO)
- KL divergence / policy shift (if RL)
- loss curves and outcome metrics (success rate, return)
- throughput (steps/sec, updates/sec)

---

## 7) Failure modes (symptom → likely cause)

- **Training diverges early**
  - lr too high; no warmup; bad initialization; exploding gradients
- **Training plateaus too early**
  - lr decayed too soon; exploration too low; batch too small
- **Learning oscillates late**
  - lr not decayed enough; entropy too high; too aggressive updates

---

## 8) Checklist: choosing your first scheduler

- Start with **warmup + cosine decay** for LR.
- In RL, schedule **entropy** (high early, lower later) if exploration collapses or stays too random.
- Change one schedule at a time and compare runs with fixed seeds.
- Always log the schedule values so you can reproduce results.

