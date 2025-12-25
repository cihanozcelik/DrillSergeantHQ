# Step 13 — First unit tests (only for what exists now)

Goal: add **fast, deterministic tests** for physics + rewards once the core is in `world.rs` (Step 12).

## 13-01 Add tests for wall bounce reflection (vx/vy flip) using fixed dt
- **What to change**: `rust/src/world.rs`
- **Technique**: `#[cfg(test)] mod tests { ... }` with direct state setup.
- **Why**: unit tests give newbies confidence without browser refresh loops.

## 13-02 Add tests for paddle collision (bounce only if falling)
- **What**: test both:
  - falling ball bounces (+reward)
  - rising ball does **not** bounce
- **Why**: this rule prevents double-bounce bugs.

## 13-03 Add tests for terminal miss + episode reset signal
- **What**: step the world until miss triggers and confirm:
  - `done = true`
  - reward includes `-1`
  - world state resets to defaults
- **Why**: episodes are the backbone of RL.

## 13-04 Add one test ensuring rewards match events (+1 bounce, -1 miss)
- **What**: verify that the **same** step that bounces gives `+1`, and the step that misses gives `-1`.
- **Why**: reward timing bugs silently break learning.

## Code (add to bottom of `rust/src/world.rs`)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn mk_world() -> World {
        World::new()
    }

    #[test]
    fn wall_bounce_flips_vx_left() {
        let mut w = mk_world();
        w.ball.r = 0.03;
        w.ball.x = 0.01; // inside left penetration
        w.ball.vx = -0.5;
        w.ball.vy = 0.0;

        let out = w.step(0.0, 0.0); // dt=0: just collision resolution
        assert!(out.wall_bounced);
        assert!((w.ball.x - w.ball.r).abs() < 1e-6);
        assert!(w.ball.vx > 0.0);
    }

    #[test]
    fn wall_bounce_flips_vy_top() {
        let mut w = mk_world();
        w.ball.r = 0.03;
        w.ball.y = 0.99;
        w.ball.vy = 0.4;

        let out = w.step(0.0, 0.0);
        assert!(out.wall_bounced);
        assert!((w.ball.y - (1.0 - w.ball.r)).abs() < 1e-6);
        assert!(w.ball.vy < 0.0);
    }

    #[test]
    fn paddle_bounce_only_when_falling() {
        let mut w = mk_world();
        // Place ball overlapping paddle.
        w.paddle.x = 0.5;
        w.paddle.y = 0.12;
        w.paddle.w = 0.20;
        w.paddle.h = 0.04;
        w.ball.r = 0.03;
        w.ball.x = 0.5;
        w.ball.y = 0.12 + 0.01;

        // Rising: should not bounce.
        w.ball.vy = 0.2;
        let out_up = w.step(0.0, 0.0);
        assert!(!out_up.bounced);
        assert_eq!(out_up.reward, 0.0);

        // Falling: should bounce.
        w.ball.vy = -0.2;
        let out_down = w.step(0.0, 0.0);
        assert!(out_down.bounced);
        assert_eq!(out_down.reward, 1.0);
        assert!(w.ball.vy > 0.0);
    }

    #[test]
    fn miss_is_terminal_and_resets_episode() {
        let mut w = mk_world();
        w.ball.r = 0.03;
        w.ball.y = -0.1;
        w.ball.vy = -0.1;
        let old_episode = w.episode_idx;

        let out = w.step(0.0, 0.0);
        assert!(out.done);
        assert_eq!(out.reward, -1.0);
        assert!(w.episode_idx > old_episode);
        // Reset defaults
        assert!((w.ball.x - 0.5).abs() < 1e-6);
        assert!((w.paddle.x - 0.5).abs() < 1e-6);
    }
}
```


