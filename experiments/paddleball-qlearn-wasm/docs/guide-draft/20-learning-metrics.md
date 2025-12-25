# Step 20 — Learning progress metrics (so it’s not mystical)

Goal: show a beginner-friendly signal that training is improving.

## 20-01 Track moving average of bounces/episode and episode length
- **What**: track a rolling average.
- **Technique**: start with exponential moving average (EMA).
- **Why**: EMA is simple and stable without storing big arrays.

## 20-02 Show last N episodes summary (or moving averages)
- **What**: optionally store last N values (N=20) for a simple “trend”.
- **Why**: makes improvement visible even if noisy.

## 20-03 Add “best so far” stats
- **What**: best bounces/episode, best episode length.
- **Why**: motivating and easy to understand.

## 20-04 Ensure metrics update rate is stable (not spamming)
- **What**: update overlay at ~5–10 Hz, not every sim step.
- **Why**: performance and readability.

## Code (metrics struct example)

```rust
#[derive(Clone, Copy, Debug)]
pub struct Metrics {
    pub ema_bounces: f32,
    pub ema_len: f32,
    pub ema_beta: f32, // e.g. 0.98
    pub best_bounces: u32,
    pub best_len: u32,
    pub last_bounces: u32,
    pub last_len: u32,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            ema_bounces: 0.0,
            ema_len: 0.0,
            ema_beta: 0.98,
            best_bounces: 0,
            best_len: 0,
            last_bounces: 0,
            last_len: 0,
        }
    }

    pub fn on_episode_end(&mut self, bounces: u32, len: u32) {
        self.last_bounces = bounces;
        self.last_len = len;
        self.best_bounces = self.best_bounces.max(bounces);
        self.best_len = self.best_len.max(len);

        let beta = self.ema_beta;
        self.ema_bounces = beta * self.ema_bounces + (1.0 - beta) * (bounces as f32);
        self.ema_len = beta * self.ema_len + (1.0 - beta) * (len as f32);
    }
}
```

Hook this where you detect episode end (miss/reset).


