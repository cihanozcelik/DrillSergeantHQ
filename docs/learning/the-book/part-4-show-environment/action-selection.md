# Action Selection and “No Teleport Jumps”

Hot-swapping a policy is only safe if your action space is *physically compatible* with continuity.

If actions directly teleport objects, then switching policies can create discontinuities that look like bugs. DrillSergeantHQ avoids that by defining actions as **targets** and **bounded impulses**, not “state edits.”

## Action selection cadence (decoupled from sim ticks)

The simulation may run at 120 Hz, but policy decisions don’t need to.

Typical pattern:

- Sim tick: 120 Hz
- Policy select: 30 Hz
- Action repeat: 4 sim steps per action

This keeps inference cost bounded and makes behavior easier to reason about.

## “No teleport” principle

**Principle**

- Policies request *intent* (desired velocity, desired angular velocity, dash intent).
- The simulator enforces physics constraints (acceleration limits, cooldowns, collisions).

This has two benefits:

- **stability**: the world remains physically plausible
- **hot-swap safety**: changing the brain doesn’t invalidate the body’s state

## A concrete action model (example)

Action components:

- movement intent: one of `{stop, forward, back, strafeL, strafeR}`
- turn intent: `{none, left, right}`
- dash intent: `{0, 1}` (with cooldown)

Discrete cartesian product gives a compact action space and fast learning:

\[
|\mathcal{A}| = 5 \times 3 \times 2 = 30
\]

## Mapping actions to physics

At each action decision:

1. Convert discrete action to target linear velocity + target angular velocity.
2. Clamp to max speed.
3. Apply acceleration/torque limits per sim tick.
4. If dash intent is on and cooldown allows, apply a bounded impulse.

**Contract**

- actions never directly write positions
- acceleration and dash impulses are bounded and deterministic
- dash has cooldown; cooldown is part of sim state

## Failure modes

- **Unbounded impulses**:  
  - symptom: “explosive” motion, training instability
  - defense: strict clamps + cooldown + test invariants

- **Action frequency mismatch**:  
  - symptom: agent appears jittery or laggy
  - defense: tune action repeat; consider smoothing or blending at the control layer

---

**Prev:** [Deterministic Fixed Timestep Simulation](fixed-timestep.md)  
**Next:** [Hot-Swapping Policies Without Breaking Reality](hot-swapping.md)


