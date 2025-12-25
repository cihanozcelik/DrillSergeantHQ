## PPO and GAE Update Loops: A Standalone, Implementation-Oriented Tutorial

### Who this is for
You want to understand PPO well enough to implement it, debug it, and tune it—without reading a paper three times. This article focuses on the **training loop**: what data you collect, what you compute, and what you optimize.

### What you’ll learn
- The roles of **policy**, **value function**, and **advantage**
- What a PPO rollout must record
- How **GAE($\lambda$)** plugs into PPO
- The PPO **clipped objective** and why it stabilizes updates
- A practical “outer loop / inner loop” pseudocode

---

## 1) PPO in one sentence

**PPO improves a policy using on-policy rollouts while preventing any single update from changing the policy too much.**

That “don’t change too much” constraint is what makes PPO stable enough to use in many settings.

---

## 2) The minimal vocabulary (plain English)

- **Policy** $\pi_\theta(a \mid s)$: a model that outputs a distribution over actions.
- **Value function** $V_\phi(s)$: a model that predicts expected future return from a state.
- **Return** $G_t$: discounted sum of rewards from time \(t\).
- **Advantage** $A_t$: how much better an action was compared to the value baseline.
- **On-policy**: the rollout data was collected using the current policy (or a very recent snapshot).

---

## 3) What your rollout must store (or you can’t do PPO)

For each time step \(t\), store:

- observation $o_t$
- action $a_t$
- reward $r_t$
- done flag $d_t$
- **old log-prob** $\log \pi_{\text{old}}(a_t \mid o_t)$
- value estimate $V(o_t)$

Why `old_logp` matters: PPO measures how the new policy differs from the old policy on *the same actions*.

---

## 4) GAE: turning rollouts into advantages and returns

After the rollout, compute advantages \(A_t\) using GAE(\(\lambda\)):

$$\delta_t = r_t + \gamma (1-d_t) V(o_{t+1}) - V(o_t)$$

$$A_t = \delta_t + \gamma \lambda (1-d_t) A_{t+1}$$

Then compute returns for the critic target:

$$R_t = A_t + V(o_t)$$

Common practice: **normalize advantages** (helps stability):

$$A \leftarrow \frac{A - \mu}{\sigma + 10^{-8}}$$

---

## 5) The PPO probability ratio (the heart of the method)

Define the ratio:

$$\text{ratio}_t = \exp\left(\log \pi_{\text{new}}(a_t \mid o_t) - \log \pi_{\text{old}}(a_t \mid o_t)\right)$$

Interpretation:

- ratio $= 1$: new policy assigns the same probability to that action
- ratio $> 1$: new policy increased the probability of that action
- ratio $< 1$: new policy decreased it

---

## 6) The clipped objective (why PPO doesn’t “blow up” as often)

If you just maximize $\text{ratio}_t A_t$, the policy can change too much in one update. PPO clips the ratio:

$$
L^{clip} = \mathbb{E}\left[\min\left(\text{ratio}_t A_t,\ \text{clip}(\text{ratio}_t, 1-\epsilon, 1+\epsilon) A_t\right)\right]
$$

Meaning:

- If the update would push ratio outside \([1-\epsilon, 1+\epsilon]\), PPO limits the incentive.
- This creates a soft “trust region” without expensive second-order math.

---

## 7) The full loss (policy + value + entropy)

In practice you optimize a combined objective:

- **Policy loss**: maximize $L^{clip}$ (often implemented as minimizing $-L^{clip}$)
- **Value loss**: fit $V(o_t)$ to $R_t$ (often MSE)
- **Entropy bonus**: encourage exploration by penalizing low-entropy policies

One common form:

$$
L = -L^{clip} + c_v \cdot \mathbb{E}[(V(o_t) - R_t)^2] - c_e \cdot \mathbb{E}[H(\pi(\cdot \mid o_t))]
$$

---

## 8) The loop structure: outer loop vs inner loop

### Outer loop (collect, then learn)

```text
repeat forever:
  rollout = collect T steps from N environments using policy π_old
  compute advantages A (GAE) and returns R
  for epoch in 1..K:
    shuffle rollout into minibatches
    for minibatch in minibatches:
      compute ratio using π_new and stored logp_old
      compute loss (policy + value + entropy)
      take an optimizer step
  set π_old ← π_new (implicitly via updated weights)
```

### Why multiple epochs?
Rollouts are expensive, so PPO reuses the rollout data for a few passes. But too many passes causes overfitting to that batch.

---

## 9) Shapes: the most common wiring bug

If you run $N$ environments for $T$ steps:

- batch size $B = N \cdot T$

Most implementations store arrays as:

- `obs[T][N][obs_dim]`
- `act[T][N]`
- `rew[T][N]`
- `done[T][N]`
- `logp_old[T][N]`
- `value[T][N]`

Then flatten `T*N` before minibatching.

---

## 10) Debugging: common mistakes and symptoms

- **You didn’t store old log-probs**
  - Symptom: ratio is wrong; training becomes unstable or meaningless.
- **You recompute “old log-probs” with the new policy**
  - Symptom: ratio collapses toward 1; learning signal disappears.
- **You forgot done masking in GAE**
  - Symptom: advantages leak across episodes; critic targets drift.
- **Too many epochs**
  - Symptom: training reward spikes then collapses; policy becomes brittle.
- **No advantage normalization**
  - Symptom: extreme sensitivity to learning rate and batch composition.

---

## 11) Practical starting hyperparameters

Reasonable defaults to start experimentation (domain-dependent):

- $\gamma = 0.99$
- $\lambda = 0.95$
- clip $\epsilon = 0.2$
- epochs $K = 3$ to $10$ (often 3–4 for large batches)
- value coefficient $c_v \approx 0.5$ to $1.0$
- entropy coefficient $c_e$ small but nonzero (e.g. 0.0–0.02)

---

## 12) Quick “did I implement PPO correctly?” checklist

- Rollout stores `logp_old` and `value` at collection time
- GAE uses done masks
- Advantages normalized
- Ratio computed from stored `logp_old` and current `logp_new`
- Multiple epochs but not excessive
- You log: approx KL, clip fraction, entropy, value loss

