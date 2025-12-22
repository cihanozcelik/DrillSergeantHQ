# Rewards and Shaping: Learnability Without Cheating

Rewards are product design disguised as math.

If the reward is too sparse, learning is slow and the user sees nothing. If shaping is naive, the agent learns degenerate tricks. The goal is not “maximize reward” in the abstract; the goal is:

> Learn a policy that looks like the behavior we intend, on a human timescale, without exploiting loopholes.

## Terminal reward (the real objective)

In the soccer-like arena, the terminal signal is simple:

- goal for: +1
- goal against: -1
- timeout: 0 (or a small negative)

This defines what “winning” means.

## Why shaping is necessary (in v1)

Early on, a random policy almost never scores. So terminal reward alone yields:

- extremely sparse learning signal
- slow early improvement

Shaping exists to accelerate the *early* stages without changing what “good” means.

## Potential-based shaping (recommended)

Potential-based shaping adds a dense reward that is provably policy-invariant under certain conditions:

Define a potential function \(\Phi(s)\) and add:

\[
r \mathrel{+}= \gamma \Phi(s') - \Phi(s)
\]

Intuition: you reward “progress” measured by a potential, but you don’t change the optimal policy the agent would learn from terminal rewards alone.

### Example potential for soccer-like play

One simple idea:

- encourage moving the ball toward the opponent goal
- discourage moving it toward your own goal

For example:

\[
\Phi(s) = -dist(ball, opp\_goal) + dist(ball, own\_goal)
\]

This tends to reward “ball progress” without directly rewarding motion for its own sake.

## Shaping pitfalls (what agents will exploit)

- **Velocity hacks**: if you reward speed, the agent learns to spin or jitter
- **Touch hacks**: if you reward “touching the ball,” the agent farms touches with no intent to score
- **Boundary hacks**: if a boundary is poorly modeled, the agent finds “legal” glitches

In engine terms: rewards are part of the simulation contract. If a loophole exists, a strong optimizer will find it.

## A practical reward design checklist

- reward terms should be bounded and comparable in scale
- shaping should not dominate terminal rewards forever (consider annealing)
- every shaping term should have a “human story” that matches intended behavior

## Observability: reward decomposition

Telemetry should include per-term reward components:

- terminal
- shaping (potential delta)
- penalties (timeouts, illegal moves if any)

If reward is a black box, debugging training becomes guesswork.

---

**Prev:** [GAE and Returns](gae.md)  
**Next:** [Self-Play and Stabilization](self-play.md)


