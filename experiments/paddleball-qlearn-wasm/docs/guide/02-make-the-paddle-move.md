# Step 02 — Make the paddle move (still no collisions)

Goal: move the paddle smoothly (no teleporting) and drive the paddle uniform from state.

## 02-01 Add paddle position to the world state (start centered)
- **What to change**: `rust/src/render.rs`
- **Technique**: add `paddle_x: f32` to `RenderState` (or a `PaddleState` struct).
- **Why**: keep state explicit; we’ll refactor later once behavior exists.

## 02-02 Implement paddle movement using `paddle_max_speed * dt` (not teleport)
- **What to change**: `rust/src/render.rs`
- **Technique**: integrate from a direction value \(-1, 0, +1\):
  - `paddle_x += dir * paddle_max_speed * dt`
- **Why**: early “random” actions should look jerky, not instantaneous jumps.

## 02-03 Clamp paddle inside the play area
- **What to change**: `rust/src/render.rs`
- **Technique**: clamp to \([paddle_w/2, 1 - paddle_w/2]\).
- **Why**: prevents the paddle from leaving the playable area and keeps collisions sane later.

## 02-04 Drive `SceneUniforms.paddle_x` from the paddle state
- **What to change**: `rust/src/render.rs`
- **Technique**: set `uniforms.paddle_x = paddle_x` before uploading.
- **Why**: the shader is pure rendering; it reads whatever you send.

## Code (covers 02-01..02-04)

```rust
// in RenderState add:
paddle_x: f32,

// in new() initialize:
paddle_x: uniforms.paddle_x,

// in update() add (for now, constant dir; Step 05 replaces with keyboard input):
let dir: f32 = 1.0; // drift right as a placeholder
let paddle_max_speed: f32 = 0.80;
self.paddle_x += dir * paddle_max_speed * dt;

let half = self.uniforms.paddle_w * 0.5;
self.paddle_x = self.paddle_x.clamp(half, 1.0 - half);
self.uniforms.paddle_x = self.paddle_x;
```


