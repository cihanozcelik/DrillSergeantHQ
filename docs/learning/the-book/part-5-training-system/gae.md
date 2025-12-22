# GAE and Returns

Rollouts give you raw experience: rewards and terminal flags. PPO needs something more structured: a signal that says *“was this action better than expected?”*

That signal is the **advantage**.

Generalized Advantage Estimation (GAE) produces advantages that are:

- lower variance than Monte Carlo returns
- less biased than pure TD(0), depending on \(\lambda\)

## Temporal-difference residual (\(\delta_t\))

\[
\delta_t = r_t + \gamma (1 - done_t) V(s_{t+1}) - V(s_t)
\]

This measures “surprise” relative to the value function.

## GAE recursion

\[
A_t = \delta_t + \gamma \lambda (1 - done_t) A_{t+1}
\]

You compute \(A_t\) backwards through time (from \(T-1\) to 0).

## Returns for value learning

A common and stable choice is:

\[
R_t = A_t + V(s_t)
\]

The value head learns to predict \(R_t\), while the policy head uses \(A_t\).

## Practical details that matter

### Normalize advantages

Advantage normalization is almost always helpful:

- subtract mean
- divide by standard deviation (+ epsilon)

This makes PPO updates less sensitive to reward scale and episode length.

### Handle terminals correctly

The \((1 - done_t)\) terms are not a footnote. If you forget them:

- advantages leak across episodes
- value learning becomes inconsistent
- training stability suffers

### Bootstrap carefully

If your rollout horizon \(T\) ends before the episode ends, you bootstrap:

- use \(V(s_T)\) as the tail value

This is normal, but it makes value correctness important—hence the “validation mode” philosophy later in the book.

## Failure modes

- **Off-by-one bugs**: wrong \(s_{t+1}\) pairing or wrong done handling  
  - symptom: value loss explodes, learning stalls

- **Bad reward scale**: huge rewards produce huge advantages  
  - symptom: ratio saturates, clipping always active, unstable updates
  - response: rescale rewards, normalize advantages, tune learning rate

GAE is where “math correctness” most often becomes “system health.” Treat it like a critical subsystem.

---

**Prev:** [PPO Objective (Clipped Surrogate)](ppo-objective.md)  
**Next:** [Rewards and Shaping: Learnability Without Cheating](rewards-and-shaping.md)


