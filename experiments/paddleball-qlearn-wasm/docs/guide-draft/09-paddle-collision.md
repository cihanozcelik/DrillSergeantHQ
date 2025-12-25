# Step 09 — Paddle collision (visible bounce)

Goal: implement paddle collision so the user can “keep the ball alive”.

## 09-01 Define paddle AABB and ball overlap test
- **What**: compute paddle bounds from center + size.
- **Technique**: AABB overlap with ball as an AABB-expanded rectangle:
  - overlap if `ball_x ± r` overlaps `px0..px1` and `ball_y ± r` overlaps `py0..py1`.
- **Why**: simplest collision math for beginners.

## 09-02 Implement “bounce only if falling” (prevents double-bounce)
- **What**: only bounce if `ball.vy < 0`.
- **Technique**: on bounce:
  - snap `ball.y = py1 + r`
  - set `vy = abs(vy)`
- **Why**: avoids repeated bouncing while the ball is already moving up.

## 09-03 Add a “paddle bounce count” counter
- **What**: increment when a paddle bounce happens.
- **Why**: numeric confirmation of correct collision.

## 09-04 Verify bounce looks plausible at different sim rates
- **What**: manual test at different `dt_fixed` values (optional).
- **Why**: catches timestep bugs early.

## Code (example patch in `rust/src/render.rs`)

Add this after wall collisions (inside the fixed-step simulation loop):

```rust
let r = self.uniforms.ball_r;
let px0 = self.paddle_x - self.uniforms.paddle_w * 0.5;
let px1 = self.paddle_x + self.uniforms.paddle_w * 0.5;
let py0 = self.uniforms.paddle_y - self.uniforms.paddle_h * 0.5;
let py1 = self.uniforms.paddle_y + self.uniforms.paddle_h * 0.5;

let overlap = self.ball.x + r >= px0
    && self.ball.x - r <= px1
    && self.ball.y + r >= py0
    && self.ball.y - r <= py1;

if overlap && self.ball.vy < 0.0 {
    self.ball.y = py1 + r;
    self.ball.vy = self.ball.vy.abs();
    self.paddle_bounce_count += 1;
    // reward later (Step 11)
}
```


