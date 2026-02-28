# Step 08 — Implement wall bounces (visible)

Goal: implement the wall bounce rules and visually confirm them.

## 08-01 Implement left/right/top wall collisions with penetration correction
- **What to change**: until Step 12 refactor, do it in `rust/src/render.rs` inside the simulation step loop.
- **Technique**: “reflect + snap”:
  - snap position back inside
  - flip sign of velocity component
- **Why**: stable and easy; avoids jitter when ball gets stuck in wall.

## 08-02 Keep bottom open (ball can fall out later)
- **What**: do **not** implement bottom bounce yet.
- **Why**: we need a “miss” terminal condition for RL.

## 08-03 Add a debug toggle to show ball velocity vector (optional overlay text is fine)
- **What**: show `vx, vy` somewhere.
- **Technique**: easiest is `log::info!` once per second; nicer is overlay text in Step 20.
- **Why**: makes it obvious that a bounce actually flipped the correct component.

## 08-04 Temporarily hardcode initial velocity for 4 runs (left/right/top/corner)
- **What**: edit the initial `vx,vy` values and refresh the page for each case.
- **Technique**: do 4 short runs:
  - A: `vx<0` hits left wall
  - B: `vx>0` hits right wall
  - C: `vy>0` hits top wall
  - D: `vx>0 && vy>0` corner / multiple bounces
- **Why**: deterministic visual verification without building extra tooling.

## 08-05 Revert to normal initial velocity (stay deterministic until Step 14 seeding)
- **What**: return to your chosen “default” `vx,vy`.
- **Why**: keep behavior reproducible while building physics.

## 08-06 Add a “wall bounce count” counter
- **What**: count how many wall bounces happened (total or per-episode).
- **Technique**: increment a counter whenever a collision branch triggers.
- **Why**: visible numeric proof the logic is firing.

## Code (example patch in `rust/src/render.rs`)

Add this inside your fixed-step simulation loop (Step 04):

```rust
let r = self.uniforms.ball_r;

let mut wall_bounced = false;

// left
if self.ball.x - r < 0.0 {
    self.ball.x = r;
    self.ball.vx = self.ball.vx.abs();
    wall_bounced = true;
}
// right
if self.ball.x + r > 1.0 {
    self.ball.x = 1.0 - r;
    self.ball.vx = -self.ball.vx.abs();
    wall_bounced = true;
}
// top
if self.ball.y + r > 1.0 {
    self.ball.y = 1.0 - r;
    self.ball.vy = -self.ball.vy.abs();
    wall_bounced = true;
}

if wall_bounced {
    self.wall_bounce_count += 1;
}
```


