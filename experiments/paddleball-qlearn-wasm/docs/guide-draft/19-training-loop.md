# Step 19 — Training loop (fast-forward sim, render stays real-time)

Goal: add Train/Pause/Evaluate modes and run multiple sim steps per render frame while still rendering once per frame.

## 19-01 Add Train/Pause/Evaluate modes
- **What**:
  - **Train**: learning updates ON
  - **Pause**: learning updates OFF, sim ON
  - **Evaluate**: learning OFF, epsilon = 0 by default
- **Why**: users must be able to pause training and see “current policy so far”.

## 19-02 Add `K` sim steps per frame in Train mode (configurable)
- **What**: integer `train_k` (e.g., 50..500).
- **Why**: learning becomes visible quickly.

## 19-03 Keep rendering once per frame from latest world state
- **What**: do NOT render per sim step.
- **Why**: performance + clarity.

## 19-04 Show training speed: steps/sec and K
- **What**: display `steps/sec` and `train_k`.
- **Why**: debug and user trust (“is it doing anything?”).

## Code (high-level wiring)

Put a small “controller” around your `World`, `Bins`, `QTable`, and `Rng`:

```rust
pub enum Mode { Manual, Agent, Train, Pause, Evaluate }

pub struct Trainer {
    pub mode: Mode,
    pub train_k: u32,
    pub epsilon: f32,
    pub alpha: f32,
    pub gamma: f32,
    pub updates: u64,
}
```

Per render frame:

```rust
let k = match trainer.mode {
    Mode::Train => trainer.train_k,
    _ => 1,
};

for _ in 0..k {
    // observe state id
    let s = discretize::state_id(&world, bins);

    // choose action
    let (a, _random) = match trainer.mode {
        Mode::Manual => (action_from_keyboard, false),
        Mode::Agent | Mode::Evaluate | Mode::Pause | Mode::Train => {
            let eps = if matches!(trainer.mode, Mode::Evaluate | Mode::Pause) { 0.0 } else { trainer.epsilon };
            qlearn::choose_action_eps_greedy(&q, s, eps, &mut rng)
        }
    };

    // step env
    let out = world.step(dt_fixed, action_dir_from_a(a));

    // learn if Train
    if matches!(trainer.mode, Mode::Train) {
        let sp = discretize::state_id(&world, bins);
        qlearn::q_update(&mut q, s, a, out.reward, sp, out.done, trainer.alpha, trainer.gamma);
        trainer.updates += 1;
    }
}

// then: render once from world state
```


