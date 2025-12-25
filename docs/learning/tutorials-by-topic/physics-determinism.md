## Physics Determinism and Time Stepping: Making Simulations Repeatable and Stable

### Who this is for
You’re building a physics or game simulation and you want two things:

- **stability** (it doesn’t explode numerically)
- **determinism** (same inputs → same outputs)

This tutorial explains fixed timesteps, integration methods, and the common sources of nondeterminism.

### What you’ll learn
- What “deterministic simulation” means
- Why variable frame time breaks physics
- Fixed timestep with an accumulator (classic approach)
- Numerical integration basics (Euler vs semi-implicit Euler)
- Practical determinism checklist

---

## 1) What does “deterministic” mean?

A simulation is **deterministic** if, given:

- the same initial state
- the same sequence of inputs (actions, controls)
- the same random seed

…it produces exactly the same results every run.

Determinism matters for:
- debugging (reproducible bugs)
- multiplayer lockstep (in some designs)
- RL training (consistent evaluation)

---

## 2) Why variable frame time is a physics trap

If you update physics with a variable \(\Delta t\) (time between frames), the simulation changes with performance:

- a slower machine produces a different trajectory
- occasional frame drops cause large \(\Delta t\) spikes
- numerical error accumulates differently each run

Even if you “multiply by dt,” variable dt changes integration error and collision resolution behavior.

---

## 3) The standard solution: fixed timestep simulation

Run physics at a fixed dt (e.g., 1/120 sec), independent of rendering.

You keep an accumulator of elapsed time and step physics in a loop:

```text
accumulator += frame_time
while accumulator >= dt:
  simulate_one_step(dt)
  accumulator -= dt
render(interpolate_if_needed)
```

This makes physics updates consistent even if rendering is variable.

---

## 4) Semi-implicit Euler (a great default)

The simplest integrator is Euler:

- $x \leftarrow x + v \Delta t$
- $v \leftarrow v + a \Delta t$

In many physics contexts, **semi-implicit Euler** (also called symplectic Euler) is more stable:

1) update velocity from acceleration
2) update position from the new velocity

This tends to conserve energy better than naive explicit Euler in many systems.

---

## 5) Determinism killers (even with fixed dt)

### Floating point nondeterminism
Different CPUs/compilers can reorder operations and produce tiny differences.

In long simulations, tiny differences can grow into large divergences (chaos).

### Multithreading
If you update entities in parallel and the order isn’t fixed, results can differ run-to-run.

### Randomness
If your RNG isn’t seeded and used deterministically, you won’t reproduce behavior.

### Collision resolution order
If collision pairs are processed in different orders, results differ.

---

## 6) Practical techniques to improve determinism

- Use a fixed timestep for simulation.
- Use a deterministic RNG with explicit seeds.
- Keep update order stable (sort entities/collisions by id).
- Avoid nondeterministic parallel reductions, or make them deterministic.
- Consider fixed-point arithmetic only if you truly need cross-platform bitwise determinism (it’s expensive).

---

## 7) A worked example: deterministic stepping plan

To make a simulation reproducible:

1) Define `dt = 1/120`.
2) Define a fixed update order for entities (by index/id).
3) Seed RNG at episode start (`seed = base + episode_id`).
4) Record user/agent inputs as discrete events with step numbers.
5) Replay by applying the same events at the same steps.

If you can replay and match checksums every 1000 steps, you’re in good shape.

---

## 8) Checklist

- Fixed dt simulation loop
- Deterministic RNG seed and usage
- Stable iteration order (entities, collisions)
- Controlled multithreading (or deterministic scheduling)
- Regression tests: “same seed → same checksum”

