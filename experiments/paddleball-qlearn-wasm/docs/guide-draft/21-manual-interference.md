# Step 21 — Manual interference during training (teachability feature)

Goal: let a user “nudge” the agent while training, to learn cause/effect.

## 21-01 Add “Manual override” toggle (user action replaces agent action)
- **What**: a boolean `manual_override`.
- **Technique**: if on, use keyboard action instead of agent action.
- **Why**: simplest interference model; easy to explain.

## 21-02 Add “Interference strength” option (manual wins vs blend)
- **What**: `strength ∈ [0..1]`.
- **Technique**:
  - strength=1: manual wins
  - strength between: blend directions (or probabilistically choose manual)
- **Why**: shows a spectrum from “coach” to “hands off”.

## 21-03 Add a visible indicator when interference is active
- **What**: overlay text “INTERFERENCE ON”.
- **Why**: avoids confusion when policy suddenly changes.

## 21-04 Verify training continues correctly with/without interference
- **What**: ensure training loop still runs and updates Q-table when in Train mode.
- **Why**: interference should not break learning plumbing.

## Code (simple interference model)

Assume:
- `agent_dir: f32` from policy
- `manual_dir: f32` from keyboard

Blend:

```rust
let strength = interference_strength.clamp(0.0, 1.0);
let final_dir = if manual_override {
    manual_dir
} else {
    // blend: strength==0 → agent, strength==1 → manual
    (1.0 - strength) * agent_dir + strength * manual_dir
};
let final_dir = final_dir.clamp(-1.0, 1.0);
```


