## Rollout Orchestration: How to Collect Training Data Efficiently in RL

### Who this is for
You understand the basic RL loop (observe → act → reward), but you’re trying to build a training system that collects a lot of experience efficiently and reliably. This tutorial is about **orchestrating rollouts**: who generates experience, how it’s batched, and how it’s fed into learning.

### What you’ll learn
- What a **rollout** is and what it must contain
- Why orchestration matters (throughput, determinism, debugging)
- Common architectures: single-threaded, vectorized, multi-worker
- How to avoid the classic bottlenecks (env stepping, inference, logging)
- A checklist for building a rollout pipeline

---

## 1) What is a rollout?

A **rollout** is a sequence of transitions collected by interacting with an environment:

$$(o_t, a_t, r_t, d_t, o_{t+1})$$

In practice, you store additional fields used by learning algorithms:

- action log-probability under the behavior policy (for PPO)
- value estimate (for advantage computation)
- episode id / step counters (for debugging)

Rollouts are your training data. If your rollouts are wrong or slow, learning will be wrong or slow.

---

## 2) Why rollout orchestration is a real engineering problem

In many RL systems, the workload is split:

- **Environment stepping** (CPU heavy, branchy)
- **Policy inference** (CPU or GPU heavy)
- **Learning updates** (often GPU heavy)

The orchestration goal is to keep all parts busy without corrupting data:

- maximize steps per second (SPS)
- keep latency bounded (fresh data)
- keep debugging manageable (reproducible runs)

---

## 3) Three rollout architectures (from simplest to scalable)

### A) Single environment, single thread (toy baseline)

- one env
- one agent
- collect steps sequentially

**Pros**: easiest to debug  
**Cons**: too slow for many problems

### B) Vectorized environments (one process, many env instances)

You run $N$ environments in a loop:

- step env[0..N-1]
- batch observations
- run policy on the batch
- apply actions back to envs

**Pros**: better CPU cache behavior; batches inference  
**Cons**: still limited by one thread/process

### C) Multi-worker rollout generation (many producers)

Multiple workers/processes each run their own vectorized env batch.

- producers generate rollouts continuously
- a learner consumes batches and updates the model

**Pros**: scales with CPU cores  
**Cons**: harder synchronization, reproducibility, and debugging

---

## 4) The two key decisions

### Decision 1: synchronous vs asynchronous

- **Synchronous**: learner waits for a full batch of fresh rollouts, then updates.
  - simpler, more stable “on-policy” behavior
- **Asynchronous**: workers keep producing while learner updates; data can become slightly stale.
  - higher throughput, but can increase instability for strict on-policy methods

### Decision 2: where inference happens

- inference in each rollout worker (distributed inference)
- inference centrally on a GPU service (workers send observations)

Distributed inference reduces traffic but may underutilize GPU; centralized inference improves GPU utilization but increases coordination overhead.

---

## 5) Worked example: a clean producer/consumer pipeline

Goal: collect $T$ steps from $N$ envs per worker.

Each rollout worker loop:

1) reset envs that are done
2) build a batch of observations
3) run policy inference → actions + logprobs + values
4) step all envs with actions
5) store transition fields into a rollout buffer
6) when buffer has $T$ steps, publish it to learner

Learner loop:

1) pull rollout batches from workers
2) compute advantages/returns
3) run optimizer updates (PPO epochs/minibatches)
4) publish updated weights to rollout workers

---

## 6) Bottlenecks and fixes

- **Env stepping dominates**
  - optimize environment code, use SoA layout, reduce branching
  - increase vectorization N per worker
- **Inference dominates**
  - batch inference; use GPU; reduce model size
  - use action repeat to reduce inference frequency
- **Communication dominates**
  - send smaller messages (ids + arrays)
  - use shared memory (when appropriate) or compress
- **Logging dominates**
  - reduce log frequency; aggregate metrics; avoid per-step logs

---

## 7) Correctness pitfalls

- **On-policy violation**
  - PPO assumes rollouts come from the current (or very recent) policy.
  - If workers use stale weights too long, training can degrade.
- **Episode boundary bugs**
  - If you don’t mask terminals correctly, advantage/return math becomes wrong.
- **Non-deterministic resets**
  - Makes debugging impossible; always support seeded resets for tests.

---

## 8) Practical checklist

- Rollout buffer schema is defined (fields, shapes, dtypes)
- Done/reset handling is correct and tested
- Inference batching is implemented
- Communication is efficient (avoid huge per-step messages)
- Learner can detect stale data and dropped batches
- Metrics: steps per second, queue depth, update time, rollout age

