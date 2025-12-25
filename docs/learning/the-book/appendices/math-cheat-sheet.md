# Math Cheat Sheet

This is a quick reference for the equations used in the training system chapters.

## GAE

$$
\delta_t = r_t + \gamma (1 - done_t) V(s_{t+1}) - V(s_t)
$$

$$
A_t = \delta_t + \gamma \lambda (1 - done_t) A_{t+1}
$$

$$
R_t = A_t + V(s_t)
$$

## PPO

$$
ratio_t = \exp(\log \pi_{new}(a_t|s_t) - \log \pi_{old}(a_t|s_t))
$$

$$
L_{clip} = -\mathbb{E}\left[\min(ratio_t \cdot A_t,\ \mathrm{clip}(ratio_t, 1-\epsilon, 1+\epsilon)\cdot A_t)\right]
$$

Typical full loss:

$$
L = L_{clip} + c_v \cdot \mathbb{E}[(V(s_t)-R_t)^2] - c_e \cdot \mathbb{E}[H(\pi(\cdot|s_t))]
$$

## Potential-based shaping

$$
r \mathrel{+}= \gamma \Phi(s') - \Phi(s)
$$

---

**Prev:** [Buffer Layouts (Draft)](buffer-layouts.md)  
**Next:** [Roadmap (Phases)](roadmap.md)


