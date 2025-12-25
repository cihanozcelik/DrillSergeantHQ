# Step 12 — (Now it’s worth it) Extract core logic out of the renderer

Goal: move the simulation into `world.rs` so it can be tested and reused by RL code.

## 12-01 Move world state + `step(dt, action)` into `rust/src/world.rs`
- **What**: create a pure-Rust `World` that owns ball + paddle + counters and exposes:
  - `reset_episode(seed?)`
  - `step(dt, action_dir) -> StepOut`
- **Technique**: keep everything numeric and deterministic; no WebGPU/WASM types.
- **Why**: isolates game logic from rendering and unlocks unit tests.

## 12-02 Keep `render.rs` responsible only for: timekeeping, calling `world.step`, filling uniforms
- **What**: renderer becomes a thin shell.
- **Why**: avoids duplicating logic in multiple places (render loop vs training loop).

## 12-03 Gate wasm-only code so `world.rs` stays pure
- **What**: keep `web_sys/wgpu` confined to renderer/WASM modules.
- **Why**: lets you run `cargo test` without a browser.

## 12-04 Keep app behavior identical after refactor
- **What**: visual behavior and counters should match pre-refactor.
- **Why**: reduces fear of refactoring for beginners.

## Code

### New file: `rust/src/world.rs` (full file)

```rust
#[derive(Clone, Copy, Debug)]
pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub r: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Paddle {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub max_speed: f32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct StepOut {
    pub reward: f32,
    pub done: bool,
    pub bounced: bool,
    pub wall_bounced: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct World {
    pub ball: Ball,
    pub paddle: Paddle,

    pub episode_idx: u32,
    pub steps_in_episode: u32,
    pub bounces_this_episode: u32,
    pub last_episode_bounces: u32,
    pub best_bounces: u32,
    pub miss_count: u32,

    pub episode_reward: f32,
    pub last_episode_reward: f32,
}

impl World {
    pub fn new() -> Self {
        Self {
            ball: Ball { x: 0.5, y: 0.65, vx: 0.20, vy: 0.15, r: 0.03 },
            paddle: Paddle { x: 0.5, y: 0.12, w: 0.20, h: 0.04, max_speed: 0.80 },

            episode_idx: 0,
            steps_in_episode: 0,
            bounces_this_episode: 0,
            last_episode_bounces: 0,
            best_bounces: 0,
            miss_count: 0,

            episode_reward: 0.0,
            last_episode_reward: 0.0,
        }
    }

    pub fn reset_episode(&mut self) {
        self.episode_idx += 1;
        self.steps_in_episode = 0;
        self.bounces_this_episode = 0;
        self.episode_reward = 0.0;

        self.ball.x = 0.5;
        self.ball.y = 0.65;
        self.ball.vx = 0.20;
        self.ball.vy = 0.15;

        self.paddle.x = 0.5;
    }

    pub fn step(&mut self, dt: f32, action_dir: f32) -> StepOut {
        let mut out = StepOut::default();
        self.steps_in_episode += 1;

        // paddle integrate
        self.paddle.x += action_dir.clamp(-1.0, 1.0) * self.paddle.max_speed * dt;
        let half = self.paddle.w * 0.5;
        self.paddle.x = self.paddle.x.clamp(half, 1.0 - half);

        // ball integrate
        self.ball.x += self.ball.vx * dt;
        self.ball.y += self.ball.vy * dt;

        // walls: left/right/top
        if self.ball.x - self.ball.r < 0.0 {
            self.ball.x = self.ball.r;
            self.ball.vx = self.ball.vx.abs();
            out.wall_bounced = true;
        }
        if self.ball.x + self.ball.r > 1.0 {
            self.ball.x = 1.0 - self.ball.r;
            self.ball.vx = -self.ball.vx.abs();
            out.wall_bounced = true;
        }
        if self.ball.y + self.ball.r > 1.0 {
            self.ball.y = 1.0 - self.ball.r;
            self.ball.vy = -self.ball.vy.abs();
            out.wall_bounced = true;
        }

        // paddle collision (bounce only if falling)
        let px0 = self.paddle.x - self.paddle.w * 0.5;
        let px1 = self.paddle.x + self.paddle.w * 0.5;
        let py0 = self.paddle.y - self.paddle.h * 0.5;
        let py1 = self.paddle.y + self.paddle.h * 0.5;
        let r = self.ball.r;

        let overlap = self.ball.x + r >= px0
            && self.ball.x - r <= px1
            && self.ball.y + r >= py0
            && self.ball.y - r <= py1;

        if overlap && self.ball.vy < 0.0 {
            self.ball.y = py1 + r;
            self.ball.vy = self.ball.vy.abs();
            out.bounced = true;
            self.bounces_this_episode += 1;
            out.reward += 1.0;
            self.episode_reward += 1.0;
        }

        // terminal miss (bottom open)
        if self.ball.y + r < 0.0 {
            out.done = true;
            out.reward -= 1.0;
            self.episode_reward -= 1.0;

            self.miss_count += 1;
            self.last_episode_bounces = self.bounces_this_episode;
            self.best_bounces = self.best_bounces.max(self.bounces_this_episode);
            self.last_episode_reward = self.episode_reward;

            self.reset_episode();
        }

        out
    }
}
```

### Wire module in `rust/src/lib.rs`

```rust
mod world;
```

### Replace ad-hoc sim in `rust/src/render.rs`

At a high level:
- store `world: crate::world::World` in `RenderState`
- each fixed step:
  - `let dir = crate::wasm_api::get_action_dir();`
  - `let out = self.world.step(self.dt_fixed, dir);`
- after stepping:
  - fill uniforms from `self.world.ball` and `self.world.paddle`


