# Step 18 — Q-learning update (learning rule)

Goal: implement the tabular Q-learning update:

\[
Q(s,a) \leftarrow Q(s,a) + \alpha \left[r + \gamma \max_{a'} Q(s',a') - Q(s,a)\right]
\]

## 18-01 Implement Bellman update with alpha/gamma and terminal handling
- **What**: if terminal, treat \(\max Q(s',\*) = 0\).
- **Why**: avoids bootstrapping from invalid terminal next state.

## 18-02 Add a deterministic unit test for one known transition update
- **What**: set Q values, apply update, check exact expected value.
- **Why**: catches sign errors and indexing bugs.

## 18-03 Add a running counter: “Q updates performed”
- **What**: `updates_count: u64`
- **Why**: makes training progress measurable.

## 18-04 Add a “Reset Q-table” button
- **What**: JS button calls `wasm_reset_qtable()` or `wasm_clear_qtable()`.
- **Why**: encourages experimentation without restarting dev server.

## Code

### Update function (add to `rust/src/qlearn.rs`)

```rust
pub fn q_update(
    q: &mut QTable,
    s: usize,
    a: usize,
    r: f32,
    sp: usize,
    done: bool,
    alpha: f32,
    gamma: f32,
) {
    let alpha = alpha.clamp(0.0, 1.0);
    let gamma = gamma.clamp(0.0, 1.0);

    let target = if done { r } else { r + gamma * q.max_q(sp) };
    let cur = q.get(s, a);
    let next = cur + alpha * (target - cur);
    q.set(s, a, next);
}
```

### Unit test (add to `rust/src/qlearn.rs`)

```rust
#[test]
fn q_update_matches_expected_math() {
    let mut q = QTable::new(2);
    // Q(s,a)=1.0, next state's max is 2.0
    q.set(0, 0, 1.0);
    q.set(1, 0, 2.0);
    q.set(1, 1, 0.0);
    q.set(1, 2, 1.0);

    let alpha = 0.5;
    let gamma = 1.0;
    let r = 0.0;
    q_update(&mut q, 0, 0, r, 1, false, alpha, gamma);
    // target = 2.0, cur=1.0 => next = 1 + 0.5*(2-1)=1.5
    assert!((q.get(0, 0) - 1.5).abs() < 1e-6);
}
```


