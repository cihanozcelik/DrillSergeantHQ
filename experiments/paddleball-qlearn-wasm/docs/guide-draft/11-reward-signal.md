# Step 11 — Reward signal (what RL will learn from)

Goal: define rewards tied to visible events.

## 11-01 Define reward: +1 on paddle bounce, -1 on miss, 0 otherwise
- **What**: a simple sparse reward.
- **Technique**:
  - on paddle bounce: reward += 1
  - on miss: reward -= 1
- **Why**: easy to understand and classic for “keep ball alive”.

## 11-02 Track cumulative reward per episode
- **What**: `episode_reward: f32`
- **Why**: sanity-check that reward aligns with bounce/miss counts.

## 11-03 Display reward stats in overlay (last episode reward, avg reward)
- **What**: log at end-of-episode; overlay later.
- **Why**: visible learning target.

## 11-04 Sanity-check: reward changes at the same moment the visual event happens
- **What**: manual check: bounce increments reward immediately; miss decrements at reset.
- **Why**: reward timing bugs break learning.

## Code (example in `rust/src/render.rs`)

At the point you detect a **paddle bounce**:

```rust
self.bounces_this_episode += 1;
self.episode_reward += 1.0;
```

At the point you detect a **miss** (before reset):

```rust
self.episode_reward -= 1.0;
self.last_episode_reward = self.episode_reward;
self.episode_reward = 0.0;
```


