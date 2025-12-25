# Step 17 — Epsilon-greedy action selection + “agent play” mode

Goal: let the agent choose actions from the Q-table, with controlled exploration.

## 17-01 Implement epsilon-greedy using the RNG
- **What**: with probability `epsilon`, choose a random action; otherwise choose argmax.
- **Technique**: sample `rng.next_f32_0_1()` and compare to epsilon.
- **Why**: exploration is necessary for learning.

## 17-02 Add a mode switch: Manual vs Agent (play-only, no learning yet)
- **What**: add a mode enum and allow “Agent” to drive paddle instead of keyboard.
- **Technique**: simplest is a `Mode` in Rust with a WASM setter (Step 19 expands this).
- **Why**: lets you observe the policy before adding learning updates.

## 17-03 Add UI control for epsilon
- **What**: add a slider or quick buttons in JS that call `wasm_set_epsilon(f32)`.
- **Why**: beginners learn by changing one knob and observing behavior.

## 17-04 Add overlay line showing current action and whether it was random/greedy
- **What**: show “action=Left (random)” vs “action=Right (greedy)”.
- **Why**: makes epsilon behavior obvious.

## Code

### Add action selection helper (example in `rust/src/qlearn.rs`)

```rust
use crate::rng::Rng;

pub fn choose_action_eps_greedy(q: &QTable, s: usize, epsilon: f32, rng: &mut Rng) -> (usize, bool) {
    let eps = epsilon.clamp(0.0, 1.0);
    let r = rng.next_f32_0_1();
    if r < eps {
        let a = (rng.next_u64() as usize) % NUM_ACTIONS;
        (a, true)
    } else {
        (q.argmax_a(s), false)
    }
}
```

### Minimal mode switch (example sketch)

In your render/training controller loop:

```rust
match mode {
    Mode::Manual => action_dir_from_keyboard,
    Mode::Agent => action_dir_from_qtable,
}
```


