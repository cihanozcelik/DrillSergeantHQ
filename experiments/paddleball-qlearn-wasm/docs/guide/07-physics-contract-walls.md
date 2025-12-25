# Step 07 — Physics contract (walls)

Goal: write the **minimum physics spec** needed to implement wall bounces next, without over-designing.

## 07-01 Write down coordinate rules (UV \([0..1]\), origin, axes) in this file
- **What**: document UV coordinates and directions.
- **Technique**: write a 5-line “contract” that everyone follows.
- **Why**: prevents “pixels vs UV” confusion and makes bounces implementable.

## 07-02 Choose explicit v1 constants: `ball_r`, `paddle_w/h`, `paddle_max_speed`
- **What**: pick numbers that match the current shader visuals.
- **Technique**: keep them consistent with `SceneUniforms` defaults (e.g., `ball_r=0.03`).
- **Why**: collision math depends on these.

## 07-03 Define wall collision rules (left/right/top) in 3–5 bullet points
- **What**: define bounces as “reflect velocity + snap inside bounds”.
- **Technique**: use “penetration correction” (snap) to avoid getting stuck.
- **Why**: simplest stable collision response for a learning sim.

## 07-04 Add one numeric example for a wall bounce (before/after)
- **What**: write one specific state + dt and the expected result.
- **Technique**: choose easy numbers (like `dt=0.1` and `vx=-1`).
- **Why**: makes the rule unambiguous for newbies.

## Spec (copy into `plan.md` if you want it in one place)

**Coordinate system**
- UV space: \(x,y \in [0,1]\)
- Origin: (0,0) bottom-left
- +x right, +y up

**Ball**
- Center at `(x,y)`, radius `r`

**Wall bounces**
- Left: if `x - r < 0` → set `x = r`, set `vx = abs(vx)`
- Right: if `x + r > 1` → set `x = 1 - r`, set `vx = -abs(vx)`
- Top: if `y + r > 1` → set `y = 1 - r`, set `vy = -abs(vy)`

**Example**
- Given `r=0.03`, `x=0.02`, `vx=-0.50` and we detect `x - r < 0`
  - After collision: `x=0.03`, `vx=+0.50`


