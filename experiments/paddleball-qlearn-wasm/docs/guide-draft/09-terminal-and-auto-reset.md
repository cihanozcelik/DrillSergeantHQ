# Step 10 — Terminal condition + auto episode reset

Goal: define “miss” and auto-reset to create episodes.

## 10-01 Define “miss” condition (ball below bottom)
- **What**: bottom is open; miss when ball is fully below screen.
- **Technique**: if `ball_y + r < 0` → miss.
- **Why**: stable and easy to reason about.

## 10-02 On miss, end the episode and reset the world automatically
- **What**: reset ball + paddle; increment episode index.
- **Technique**: call a small `reset_episode()` function that sets state back to defaults.
- **Why**: keeps renderer running; only state resets.

## 10-03 Add counters: misses, bounces/episode, best bounces
- **What**: track `miss_count`, `bounces_this_episode`, `best_bounces`.
- **Why**: metrics are the foundation for “learning progress”.

## 10-04 Display “last episode bounces” in overlay
- **What**: at minimum, log at episode end.
- **Why**: immediately answers “am I improving?”.

## Code (example patch in `rust/src/render.rs`)

Inside the fixed-step loop, after collisions:

```rust
let r = self.uniforms.ball_r;
if self.ball.y + r < 0.0 {
    self.miss_count += 1;
    self.last_episode_bounces = self.bounces_this_episode;
    self.best_bounces = self.best_bounces.max(self.bounces_this_episode);
    self.episode_idx += 1;

    // reset state
    self.bounces_this_episode = 0;
    self.ball = BallState { x: 0.5, y: 0.65, vx: 0.20, vy: 0.15 };
    self.paddle_x = 0.5;
}
```


