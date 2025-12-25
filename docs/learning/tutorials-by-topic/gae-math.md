## Generalized Advantage Estimation (GAE): What It Is and Why It Helps

### Who this is for
You’re learning policy-gradient reinforcement learning (PPO, A2C/A3C, TRPO) and you keep seeing “GAE($\lambda$)”. This tutorial explains the terminology and the math in a way you can implement without guesswork.

### What you’ll learn
- What an **advantage** is and why it matters
- What the **TD error** $\delta_t$ means
- What **GAE($\lambda$)** computes and what $\lambda$ controls
- A step-by-step recipe to compute advantages from a rollout

---

## 1) The goal: estimate “was this action better than expected?”

Policy-gradient methods push the policy to repeat actions that turned out well. A common training signal is the **advantage**:

$$A_t = Q(s_t, a_t) - V(s_t)$$

- $Q(s_t, a_t)$: expected return if you take action $a_t$ in state $s_t$
- $V(s_t)$: expected return from state $s_t$ under the current policy

Interpretation:
- $A_t > 0$: action was better than expected → increase its probability
- $A_t < 0$: action was worse than expected → decrease its probability

The challenge: we don’t know $Q$ exactly, so we approximate $A_t$ from sampled rollouts.

---

## 2) The building blocks: returns, values, and TD error

### Return
The discounted return from time $t$ is:

$$G_t = r_t + \gamma r_{t+1} + \gamma^2 r_{t+2} + \cdots$$

where $\gamma \in [0, 1)$ is the **discount factor**.

### Value function
$V(s_t)$ is a learned estimate of $G_t$ (expected future return).

### TD error (one-step “surprise”)
The TD error compares what the value function predicted vs. what happened next:

$$\delta_t = r_t + \gamma (1-d_t) V(s_{t+1}) - V(s_t)$$

where $d_t$ is 1 if the episode ended at step $t$, else 0.

If $\delta_t$ is positive, things went better than $V(s_t)$ predicted.

---

## 3) The bias–variance tradeoff (why GAE exists)

You can estimate advantage in different ways:

### Monte Carlo advantage (low bias, high variance)

$$A_t \approx G_t - V(s_t)$$

It uses full returns, but it’s noisy—especially for long episodes.

### 1-step TD advantage (higher bias, lower variance)

$$A_t \approx \delta_t$$

Less noisy, but depends heavily on how accurate $V$ is.

GAE gives you a principled way to interpolate between these two.

---

## 4) GAE($\lambda$) in one sentence

**GAE computes advantage by summing TD errors forward in time, discounted by $\gamma\lambda$.**

Definition:

$$A_t^{GAE(\lambda)} = \sum_{l=0}^{\infty} (\gamma \lambda)^l \, \delta_{t+l}$$

In practice you compute it backward over a finite rollout.

---

## 5) The practical recursion (what you implement)

For a rollout of length \(T\), compute backwards:

$$A_{T-1} = \delta_{T-1}$$

$$A_t = \delta_t + \gamma \lambda (1-d_t)\, A_{t+1}$$

### What $\lambda$ does
- $\lambda = 0$: $A_t = \delta_t$ (pure 1-step TD)
- $\lambda \to 1$: uses longer chains of TD errors (approaches Monte Carlo style)

So $\lambda$ is a **bias–variance knob**.

---

## 6) Worked example (conceptual)

Suppose $T=3$ with steps $t=0,1,2$, and the episode ends at $t=2$. Then:

- $A_2 = \delta_2$ (no bootstrapping past terminal)
- $A_1 = \delta_1 + \gamma\lambda A_2$
- $A_0 = \delta_0 + \gamma\lambda A_1$

The key intuition: if the future was surprisingly good, earlier actions get credit too—scaled by $\gamma\lambda$.

---

## 7) Implementation checklist (the “don’t mess this up” list)

- **Mask terminals** with $(1-d_t)$ so you don’t bootstrap across episode boundaries.
- Keep values aligned: you need $V(s_t)$ and $V(s_{t+1})$.
- Be consistent about array shapes: `T x N` (time-major) is common for batched rollouts.
- Advantage normalization (common in PPO):
  - $$A \leftarrow \frac{A - \mu}{\sigma + 10^{-8}}$$

---

## 8) Practical defaults

Common starting values:

- $\gamma = 0.99$
- $\lambda = 0.95$

Then tune:

- more stability / less variance → lower $\lambda$ (e.g., 0.9)
- more “long-horizon credit” → higher $\lambda$ (e.g., 0.97–0.99)

