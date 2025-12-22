# The “WTF Map”: One Tick Through the Whole System

This chapter exists for one reason: to answer the reader’s question—

> What the hell is going on?

We’ll walk the system once end-to-end. Not at “kernel math” depth, not at “physics impulse” depth—just enough to form a **mental model** you can reuse everywhere else.

## The cast (four runtimes)

DrillSergeantHQ is one product, but it behaves like four cooperating programs:

- **Main Thread (UI)**: buttons, sliders, input capture, displaying metrics
- **Render/Eval Worker**: the show match simulation + rendering loop
- **Trainer Worker**: PPO learner + checkpoint publisher
- **Rollout Worker Pool (N)**: batched headless environments producing experience

## The three hot channels (where the bytes actually flow)

There are exactly three “big” data paths worth memorizing:

1. **Control messages** (small, infrequent): UI → workers  
2. **Weights** (big, periodic): Trainer → Render/Eval (SAB double buffer)  
3. **Rollouts** (huge, continuous): Rollouts → Trainer (SAB ring buffer)  

Everything else is secondary.

## End-to-end walk: from “Train” to “Smarter”

### Step 0: The user hits “Train”

On the UI thread:

- UI sends `START_TRAINING { …hyperparams… }` to the Trainer worker.
- UI continues rendering its own DOM; it does **not** run training.

### Step 1: The show match keeps running (unchanged)

In the Render/Eval worker:

- The show environment advances with a **fixed timestep** (e.g. 120 Hz).
- Rendering happens at a **frame rate** (e.g. 60 FPS).
- Policy inference runs at a lower cadence (e.g. 10–30 Hz via action repeat).

This loop is sacred. Training is not allowed to steal its budget.

### Step 2: Rollout workers generate experience (the “factory”)

In each rollout worker:

- Thousands of environment instances step forward in a tight SoA batch.
- Each env produces: observations, actions, rewards, dones, log-probs, values.
- The worker writes this experience into a **SharedArrayBuffer ring buffer**.

The point is throughput: keep CPU cores busy without copying data through postMessage.

### Step 3: The Trainer consumes rollouts and runs PPO updates

In the Trainer worker:

- It reads rollout chunks from the ring buffer.
- It computes **GAE advantages** and returns.
- It runs **PPO** updates (often using WebGPU compute for the heavy math).
- It produces a new set of weights (a checkpoint).

### Step 4: The Trainer publishes a checkpoint (without blocking anyone)

The Trainer writes weights into the inactive half of a **double buffer** in SAB:

- write weights to inactive buffer
- atomically flip `active_idx`
- atomically increment `version`

No large messages. No copies. Just a pointer flip and a version number.

### Step 5: The show match hot-swaps weights (without restarting)

In the Render/Eval worker:

- On a timer or version change, it checks `version`.
- If newer: it reads `active_idx` and switches its policy’s weight pointer.

The simulation state is unchanged. The only thing that changes is the agent’s “brain.”

### Step 6: The user observes behavior improve

Because:

- the show match never stopped
- the hot-swap never copied megabytes through messages
- updates are frequent enough to be noticeable

## The one diagram you should remember

```text
 UI (Main Thread)               Show (Render/Eval Worker)
  - controls/input     small     - fixed-timestep sim
  - metrics display  messages    - rendering loop
         |                         - inference + hot-swap
         |                                  ^
         v                                  | SAB weights (double buffer)
 Trainer Worker  <---- SAB rollouts ----  Rollout Workers (N)
  - PPO/GAE update        ring buffer      - batched headless envs
  - publish checkpoints                  - write experience continuously
```

## How later chapters “snap onto” this map

- **Worker topology**: explains why these four runtimes exist and what each is allowed to do.
- **Timing and loops**: defines the budgets that prevent training from ruining the show match.
- **Protocols**: defines the message contracts for small control/telemetry.
- **Shared memory strategy**: defines the SAB contracts that make the system fast and predictable.

---

**Prev:** [Part II — System Architecture](README.md)  
**Next:** [Worker Topology and Responsibilities](worker-topology.md)



