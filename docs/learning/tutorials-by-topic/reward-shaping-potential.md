## Reward Shaping (and Potential-Based Shaping): How to Help an Agent Learn Without Teaching It to Cheat

### Who this is for
You’re building a reinforcement learning environment and your agent is not learning. People tell you to “shape the reward,” but you worry (correctly) that shaping can create bizarre unintended behaviors. This tutorial teaches reward shaping as an engineering discipline, not a bag of hacks.

### What you’ll learn
- What “reward shaping” actually means
- Why sparse rewards are hard
- The main reward shaping failure modes (reward hacking)
- What **potential-based reward shaping** is and why it’s safer
- A practical workflow for shaping rewards responsibly

---

## 1) What is reward shaping?

In RL, the agent learns by maximizing expected cumulative reward:

$$
\mathbb{E}\left[\sum_{t=0}^{\infty} \gamma^t r_t\right]
$$

**Reward shaping** means adding extra reward signals that make learning easier—usually by giving the agent “hint rewards” on the way to a sparse goal.

Examples:
- small reward for moving toward the goal
- small penalty for wasting time
- bonus for completing subtasks

Shaping is often necessary because sparse rewards can be too rare for exploration to stumble upon.

---

## 2) Why sparse rewards are hard (intuition)

If the only reward is “+1 at the end if you win,” then early in training:

- the agent almost never wins
- gradients are based on near-zero signal
- learning looks dead for a long time

Shaping adds intermediate signal so the agent can learn “directionally correct” behavior sooner.

---

## 3) The main danger: reward hacking (a.k.a. specification gaming)

When you add shaping, you create a new objective. The agent will optimize it literally, not the way you intended.

Common failure patterns:

- **Proxy exploitation**: agent maximizes the proxy (shaped reward) while ignoring the real goal.
- **Looping behaviors**: agent learns to farm reward repeatedly (oscillate, spin, bounce).
- **Stalling**: agent avoids terminal states because living yields more shaped reward than finishing.

Engineering rule:
> If a shaped reward can be gained without progressing toward the real objective, the agent will try it.

---

## 4) Discount factor $\gamma$ (why it changes shaping behavior)

$\gamma$ controls how much the agent values the future:

- $\gamma$ near 1 → long-horizon planning
- lower $\gamma$ → short-term focus

Shaping terms interact with $\gamma$ because the agent cares about **discounted** reward. A shaping signal that looks “small” per step can dominate total reward over long episodes.

Practical implication:
- If you add per-step shaping, be careful: small per-step rewards can add up to a lot.

---

## 5) Potential-based reward shaping (the safer method)

Potential-based shaping adds a shaped reward defined by a potential function \(\Phi(s)\):

$$r'_t = r_t + \gamma \Phi(s_{t+1}) - \Phi(s_t)$$

Key property (why people like it):
- Under common assumptions, it **does not change which policies are optimal**—it just changes learning dynamics.

Intuition:
- You’re not adding “extra goals.”
- You’re adding a **difference of potentials** that behaves like a shaping gradient field.

---

## 6) Worked example: shaping “reach the goal”

Environment:
- agent moves in a 2D plane
- goal is a target point
- terminal reward: +1 when within radius

Sparse reward:
- $r_t = 1$ at success, else 0

Potential function:
- $\Phi(s) = -\text{distance}(\text{agent}, \text{goal})$

Shaped reward:

$$
r'_t = r_t + \gamma(-d_{t+1}) - (-d_t) = r_t + (d_t - \gamma d_{t+1})
$$

If the agent moves closer (distance decreases), $r'_t$ tends to be positive.

This helps the agent learn “move toward goal” quickly, while keeping the original terminal reward as the real objective.

---

## 7) Shaping workflow (what professionals do)

### Step 1: Write the true objective as a test
Example: “success rate must increase” or “episode return correlates with success.”

### Step 2: Add shaping that is hard to game
Prefer:
- monotonic potentials (distance-to-goal)
- progress measures that can’t be farmed in place

Avoid:
- rewards for being in a region without requiring progress
- rewards for oscillations (e.g., “speed” without direction)

### Step 3: Keep shaping small relative to terminal outcomes
If terminal success is +1, don’t let shaping contribute +10 over an episode unless that’s intentional.

### Step 4: Monitor behavior, not just reward
Track:
- success rate
- episode length
- distinct states visited
- failure mode videos (even simple renderings)

If reward goes up but success does not, you likely created a proxy exploit.

---

## 8) Common shaping patterns (with warnings)

- **Time penalty** (`-0.01` per step)
  - Helps avoid stalling; can also make exploration harder if too strong.
- **Energy penalty** (penalize large actions)
  - Encourages smooth control; can lead to “do nothing” policies if too strong.
- **Progress reward** (delta distance, delta potential)
  - Often great; still needs anti-oscillation design.

---

## 9) Quick checklist

- Does shaped reward correlate with real success?
- Can the agent farm shaped reward without progressing?
- Is shaping bounded so it doesn’t dominate terminal goals?
- Did you test multiple seeds and observe behavior?
- Can you remove shaping later and still solve the task?

