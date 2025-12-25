# Step 04 — Fixed timestep accumulator (render once, simulate N steps)

Goal: keep physics stable by stepping in fixed dt chunks, while rendering once per frame.

## 04-01 Add `dt_fixed` and an accumulator
- **What to change**: `rust/src/render.rs`
- **Technique**: add:
  - `accum_s: f32`
  - `dt_fixed: f32` (e.g. 1/120)
- **Why**: fixed dt makes collisions + RL learning signals consistent.

## 04-02 Step the world 0..N times per render frame
- **What to change**: `rust/src/render.rs`
- **Technique**: loop while `accum_s >= dt_fixed` and call sim step.
- **Why**: decouples render FPS from simulation behavior.

## 04-03 Cap max sim steps per frame
- **What to change**: `rust/src/render.rs`
- **Technique**: `max_steps` guard (e.g. 16).
- **Why**: prevents frame spirals on slow machines.

## 04-04 Render once per frame from latest world state
- **What to change**: `rust/src/render.rs`
- **Technique**: after sim loop, copy world → uniforms once and upload once.
- **Why**: rendering per sub-step wastes GPU and makes the animation look “fast-forward flickery”.

## Code (covers 04-01..04-04)

```rust
// RenderState fields
accum_s: f32,
dt_fixed: f32,

// new()
accum_s: 0.0,
dt_fixed: 1.0 / 120.0,

// update(): accumulate real dt, step fixed dt multiple times
self.accum_s += dt;
let mut steps = 0;
let max_steps = 16;
while self.accum_s >= self.dt_fixed && steps < max_steps {
    steps += 1;
    self.accum_s -= self.dt_fixed;

    // simulate using dt_fixed
    self.ball.x += self.ball.vx * self.dt_fixed;
    self.ball.y += self.ball.vy * self.dt_fixed;
    // paddle integration here too
}

// render once per frame from latest state
self.uniforms.ball_x = self.ball.x;
self.uniforms.ball_y = self.ball.y;
self.uniforms.paddle_x = self.paddle_x;
```


