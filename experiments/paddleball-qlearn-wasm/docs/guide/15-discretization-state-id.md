# Step 15 — Discretization (state_id)

Goal: map continuous world state (floats) into a finite integer `state_id` for a Q-table.

## 15-01 Define bins for ball_x, ball_y, ball_vx, ball_vy, paddle_x
- **What**: pick small counts (learning speed > precision).
- **Technique**: start with something like:
  - x/y: 12 bins
  - vx/vy: 7 bins each (map \([-vmax, vmax]\))
  - paddle_x: 12 bins
- **Why**: too many bins makes learning look “broken” (sparse updates).

## 15-02 Implement `state_id(world) -> usize` and `num_states()`
- **What to add**: `rust/src/discretize.rs`
- **Technique**: base-N packing:
  - `id = (((((bx)*By + by)*Bvx + bvx)*Bvy + bvy)*Bpx + bpx)`
- **Why**: fast, deterministic, and easy to test.

## 15-03 Add tests: state_id in range; small state changes affect id
- **What**: validate bounds and sensitivity.
- **Why**: prevents out-of-bounds Q-table access.

## 15-04 Add an overlay debug line showing current `state_id`
- **What**: show the number (console log first; overlay later).
- **Why**: helps debug “is the agent seeing different states?”

## Code

### New file: `rust/src/discretize.rs` (full file)

```rust
use crate::world::World;

#[derive(Clone, Copy, Debug)]
pub struct Bins {
    pub ball_x: usize,
    pub ball_y: usize,
    pub ball_vx: usize,
    pub ball_vy: usize,
    pub paddle_x: usize,
    pub vmax: f32, // used for velocity binning
}

impl Bins {
    pub fn num_states(&self) -> usize {
        self.ball_x * self.ball_y * self.ball_vx * self.ball_vy * self.paddle_x
    }
}

fn bin_0_1(v: f32, bins: usize) -> usize {
    if bins <= 1 { return 0; }
    let v = v.clamp(0.0, 1.0);
    let idx = (v * (bins as f32)) as usize;
    idx.min(bins - 1)
}

fn bin_neg_vmax(v: f32, bins: usize, vmax: f32) -> usize {
    if bins <= 1 { return 0; }
    let vmax = vmax.max(1e-6);
    let v = v.clamp(-vmax, vmax);
    let t = (v + vmax) / (2.0 * vmax); // map to [0,1]
    bin_0_1(t, bins)
}

pub fn state_id(w: &World, b: Bins) -> usize {
    let bx = bin_0_1(w.ball.x, b.ball_x);
    let by = bin_0_1(w.ball.y, b.ball_y);
    let bvx = bin_neg_vmax(w.ball.vx, b.ball_vx, b.vmax);
    let bvy = bin_neg_vmax(w.ball.vy, b.ball_vy, b.vmax);
    let bpx = bin_0_1(w.paddle.x, b.paddle_x);

    // base-N packing
    let mut id = bx;
    id = id * b.ball_y + by;
    id = id * b.ball_vx + bvx;
    id = id * b.ball_vy + bvy;
    id = id * b.paddle_x + bpx;
    id
}
```

### Tests (add to `rust/src/discretize.rs`)

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::World;

    #[test]
    fn state_id_in_range() {
        let w = World::new();
        let b = Bins { ball_x: 12, ball_y: 12, ball_vx: 7, ball_vy: 7, paddle_x: 12, vmax: 1.0 };
        let id = state_id(&w, b);
        assert!(id < b.num_states());
    }

    #[test]
    fn state_id_changes_when_ball_moves_bins() {
        let mut w = World::new();
        let b = Bins { ball_x: 12, ball_y: 12, ball_vx: 7, ball_vy: 7, paddle_x: 12, vmax: 1.0 };
        let a = state_id(&w, b);
        w.ball.x = (w.ball.x + 0.2).min(1.0);
        let c = state_id(&w, b);
        assert_ne!(a, c);
    }
}
```


