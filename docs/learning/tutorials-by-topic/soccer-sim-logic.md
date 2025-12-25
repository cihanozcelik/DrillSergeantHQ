## 2D Game Physics Simulation Logic (Soccer‑Like Arena): A Standalone Tutorial

### Who this is for
You want to build a simple 2D physics-driven game (top-down “soccer-ish” arena, air hockey, bumper robots) with believable motion and collisions. You don’t want a full engine—you want to understand the logic well enough to implement a minimal simulation yourself.

### What you’ll learn
- How to structure a 2D simulation update loop
- Why **semi-implicit Euler** is a good default integrator
- Collision detection and response basics:
  - circle vs wall
  - circle vs oriented box (OBB) at a high level
- How to maintain stability (invariants, clamping)
- What “state continuity” means when control logic changes

---

## 1) The simulation loop (the skeleton)

A physics simulation is usually:

1) Apply forces/controls (acceleration, impulses)
2) Integrate motion (update velocity and position)
3) Detect collisions
4) Resolve collisions (separate + apply impulses)
5) Apply damping/clamps (optional but common)

Run this at a fixed timestep for stability (see fixed dt discussion in determinism tutorials).

---

## 2) Semi-implicit Euler (why it’s used everywhere)

Given position $p$, velocity $v$, acceleration $a$, timestep $\Delta t$:

1) update velocity:
$$
v \leftarrow v + a \Delta t
$$
2) update position with the new velocity:
$$
p \leftarrow p + v \Delta t
$$

This method is simple and often more stable than “explicit Euler” in game-like settings.

---

## 3) Circle vs wall collisions (practical approach)

For a circle (center $c$, radius $r$) and a wall represented as a line/segment with a normal vector $n$:

### Detection idea
Compute signed distance from circle center to wall:

- if distance < r → penetration

### Resolution idea (minimum translation)
Push the circle out along the normal:

$$c \leftarrow c + (r - d) n$$

Then reflect or impulse-correct velocity along the collision normal.

### Velocity response (basic)
Split velocity into normal and tangential components:

- $v_n = (v \cdot n)n$
- $v_t = v - v_n$

Reflect the normal component with restitution $e \in [0,1]$:

$$v \leftarrow v_t - e v_n$$

Add friction by scaling tangential velocity:

$$v \leftarrow (1-\mu) v_t - e v_n$$

This is a simple but effective model for many arcade physics games.

---

## 4) Circle vs OBB (oriented box) collisions (high-level)

An **OBB** is a rotated rectangle. Handling circle vs OBB is a classic game-physics task.

A common method:

1) Transform the circle center into the box’s local space (rotate by \(-\theta\)).
2) Clamp that local point to the rectangle extents to get the closest point on the box.
3) Compute vector from closest point to circle center.
4) If length < radius → collision.

Response:
- Push the circle out along the collision normal (from box to circle).
- Apply an impulse/reflection similar to wall case.

You can implement this without complex geometry libraries if you are careful with coordinate transforms and clamping.

---

## 5) Stability tools: the “boring” parts that prevent explosions

Physics engines often include guardrails:

- clamp maximum speed
- clamp maximum angular velocity
- apply damping (air resistance)
- cap penetration correction per step
- limit impulses to prevent huge bounces from numerical artifacts

These don’t make physics “more correct,” but they make it playable and stable.

---

## 6) Physics invariants and sanity checks

Useful checks:
- no NaNs/Infs in positions/velocities
- energy should not increase without a source (unless you intentionally add impulses)
- objects should remain within bounds after collision resolution

In practice, you use these checks to detect bugs in collision resolution or integration.

---

## 7) State continuity when control logic changes

Sometimes you change the “brain” controlling an object (player input, AI policy, new controller). **State continuity** means:

- you do not reset physics state when the controller changes
- you only change the forces/inputs applied going forward

Why it matters:
- the world remains physically consistent
- no teleport jumps or discontinuities

This is especially important in systems where controllers update frequently.

---

## 8) Checklist for building a minimal 2D arena sim

- Fixed timestep loop (dt)
- Semi-implicit Euler integration
- Collision detection + resolution for your key shapes
- Restitution and friction modeled simply
- Stability clamps and damping
- Debug visualizations (draw normals, penetration depth)
- Invariant checks (NaN guard, bounds guard)

