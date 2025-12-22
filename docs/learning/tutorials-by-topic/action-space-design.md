## Action Space Design (Reinforcement Learning): A Standalone Guide

### Who this is for
You’re building (or studying) a reinforcement learning agent and you’ve hit the question: **“What should the agent be allowed to do?”** This article explains action spaces from the ground up, with a concrete example you can copy.

### What you’ll learn
- What an **action space** is (in plain English)
- The difference between **discrete** and **continuous** actions
- What **action repeat / frame-skip** means and why it matters
- How to design actions that **don’t break physics** (no teleporting)
- A step-by-step recipe for designing your own action space

---

## 1) What is an action space?

In reinforcement learning, an **agent** repeatedly:

1) observes the world (the **state** or an **observation**)  
2) chooses an **action**  
3) receives a **reward**  
4) repeats

The **action space** is simply the set of all actions the agent is allowed to choose from.

- If the action space is too small, the agent can’t express good behavior.
- If it’s too big or too precise, learning becomes slow and unstable.

Think of it like teaching someone to cook:

- If you only allow “boil” and “fry,” they can’t bake a cake.
- If you allow “set stove to exactly 173.2°C,” beginners flail.

Good action spaces are **expressive**, **learnable**, and **physically valid**.

---

## 2) Terminology (so the rest makes sense)

- **Discrete actions**: choose one option from a list (like pressing a button).
  - Example: `{left, right, jump, do_nothing}`
- **Continuous actions**: output a real number (like turning a knob).
  - Example: steering angle in \([-1, 1]\)
- **Policy**: the agent’s decision rule (a function that maps observations → actions).
- **Action frequency**: how often the policy picks a new action (e.g., 10 times per second).
- **Simulation step**: how often physics/world updates run (e.g., 120 updates per second).
- **Action repeat / frame-skip**: reuse the same action for multiple simulation steps.

---

## 3) Discrete vs. continuous: how to choose

### Discrete action spaces
The agent picks one item from a menu.

**Great when:**
- You want a stable, easy-to-debug setup
- You can describe behavior as combinations of a few “modes”
- You have cooldowns/abilities that are naturally on/off

**Hard when:**
- You need extremely smooth control (e.g., fine steering, torque control)
- Your discrete menu becomes huge

### Continuous action spaces
The agent outputs numbers (often one per control dimension).

**Great when:**
- The task is fundamentally analog (steering, throttle, aiming)
- You need smoothness

**Hard when:**
- Exploration is difficult (random noise in continuous space can be unhelpful)
- You must enforce constraints (rate limits, saturations) carefully

### A practical beginner rule
- Start with **discrete** if you can; upgrade to continuous when discrete control can’t express what you need.

---

## 4) The #1 design mistake: actions that “teleport”

If an action directly sets position (“move to X”), the agent will often discover exploits:

- moving through walls
- instant stopping
- impossible acceleration

Instead, actions should control **forces**, **accelerations**, or **target velocities**—things that a physics system can realistically apply.

### The fix: target-based controls
Rather than “set position,” use:
- **target velocity** (move intent)
- **target angular velocity** (turn intent)
- **bounded impulse with cooldown** (dash/jump-like abilities)

This keeps learning grounded in real dynamics.

---

## 5) Worked example: designing actions for a top-down character

Imagine a simple top-down character that can move and turn.

### Step A: list what the character can do
- Move in 2D (forward/back/left/right)
- Turn left/right
- Optional “dash” ability on a cooldown

### Step B: choose discrete factorized actions (easy to learn)
We make three small menus and combine them:

- Move: `{stop, forward, back, left, right}` (5)
- Turn: `{none, left, right}` (3)
- Dash: `{no, yes}` (2)

Total actions: \(5 \times 3 \times 2 = 30\)

### Step C: map actions to *targets*, not teleports
When the agent selects an action, we convert it into targets:

- Move choice → target velocity vector (clamped)
- Turn choice → target angular velocity (clamped)
- Dash choice → if cooldown ready, apply a bounded impulse

### Step D: apply rate limits (smoothness)
Even target velocities can change too abruptly. Add an acceleration limit:

\[
v \leftarrow v + \mathrm{clip}(v_{target} - v,\; -a_{max}\Delta t,\; a_{max}\Delta t)
\]

Now the agent can’t instantly flip from full-left to full-right.

### Step E: choose action frequency (and action repeat)
Let physics run fast (e.g., 120 Hz), but choose actions slower (e.g., 30 Hz):

- **simulation**: 120 Hz  
- **policy decisions**: 30 Hz  
- **action repeat**: 4 simulation steps per chosen action

This reduces jitter and makes learning easier.

---

## 6) Action repeat (frame-skip) explained plainly

**Action repeat** means: “Pick an action, then keep doing it for a short while.”

Why it helps:
- fewer decisions → easier credit assignment
- smoother behavior
- faster training (fewer policy evaluations)

How it hurts (if too large):
- agent feels sluggish
- can’t react to fast events (collisions, sudden threats)

**Typical starting range**: repeat 2–6 steps, then tune by observing behavior.

---

## 7) Common problems and how to diagnose them

- **Learning is extremely slow**
  - Likely cause: too many actions, or actions too “fine-grained”
  - Fix: reduce action count, factorize, add action repeat
- **Agent jitters or vibrates**
  - Likely cause: policy changes action too often
  - Fix: increase action repeat, add acceleration limits
- **Agent finds “physics hacks”**
  - Likely cause: actions control state directly (teleport)
  - Fix: control forces/targets; clamp and rate-limit
- **Agent seems incapable of winning**
  - Likely cause: action space missing an important capability
  - Fix: add one capability at a time; keep changes minimal and testable

---

## 8) A reusable design recipe (copy this)

When you design an action space, write answers to these questions:

- **Capability list**: What must the agent be able to do?
- **Representation**: Discrete / continuous / hybrid?
- **Decision rate**: How often does it choose?
- **Mapping**: How do actions become physically valid controls?
- **Constraints**: What clamps, cooldowns, and rate limits exist?
- **Debug plan**: How will you tell if the agent is stuck because of actions vs. reward vs. observations?

---

## 9) Exercises (to make this “stick”)

- **Exercise 1**: Design a 12–40 action discrete table for a car: steer, throttle, brake.
- **Exercise 2**: Take a continuous steering control and add a steering *rate limit*. Observe how it changes behavior.
- **Exercise 3**: Reduce your action space by 50% and measure whether learning improves.


