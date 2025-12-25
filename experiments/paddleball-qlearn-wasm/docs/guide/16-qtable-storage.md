# Step 16 — Q-table storage + helpers

Goal: implement the “model” as a Q-table: `Q[state, action]`.

## 16-01 Define action set: Left / Stay / Right (3 actions)
- **What**: hardcode 3 actions for the first version.
- **Why**: tiny action space makes learning fast and visible.

## 16-02 Implement Q-table storage: `q[s * A + a]`
- **What to add**: `rust/src/qlearn.rs`
- **Technique**: 1D flat `Vec<f32>` for cache-friendly access.
- **Why**: simplest, fastest, easy to serialize later.

## 16-03 Implement helpers: `max_q(s)`, `argmax_a(s)`
- **What**: pure functions that read the table.
- **Why**: makes policy selection and updates clean.

## 16-04 Add tests for indexing correctness
- **What**: verify that writes/reads hit expected indices.
- **Why**: prevents subtle off-by-one learning bugs.

## Code

### New file: `rust/src/qlearn.rs` (full file)

```rust
#[derive(Clone, Copy, Debug)]
pub enum Action {
    Left = 0,
    Stay = 1,
    Right = 2,
}

pub const NUM_ACTIONS: usize = 3;

#[derive(Clone, Debug)]
pub struct QTable {
    pub num_states: usize,
    pub q: Vec<f32>, // length = num_states * NUM_ACTIONS
}

impl QTable {
    pub fn new(num_states: usize) -> Self {
        Self { num_states, q: vec![0.0; num_states * NUM_ACTIONS] }
    }

    #[inline]
    pub fn idx(&self, s: usize, a: usize) -> usize {
        debug_assert!(s < self.num_states);
        debug_assert!(a < NUM_ACTIONS);
        s * NUM_ACTIONS + a
    }

    #[inline]
    pub fn get(&self, s: usize, a: usize) -> f32 {
        self.q[self.idx(s, a)]
    }

    #[inline]
    pub fn set(&mut self, s: usize, a: usize, v: f32) {
        let i = self.idx(s, a);
        self.q[i] = v;
    }

    pub fn max_q(&self, s: usize) -> f32 {
        let base = s * NUM_ACTIONS;
        let mut m = self.q[base];
        for i in 1..NUM_ACTIONS {
            m = m.max(self.q[base + i]);
        }
        m
    }

    pub fn argmax_a(&self, s: usize) -> usize {
        let base = s * NUM_ACTIONS;
        let mut best_a = 0;
        let mut best_v = self.q[base];
        for a in 1..NUM_ACTIONS {
            let v = self.q[base + a];
            if v > best_v {
                best_v = v;
                best_a = a;
            }
        }
        best_a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indexing_is_correct() {
        let mut t = QTable::new(10);
        t.set(3, 2, 7.5);
        assert_eq!(t.get(3, 2), 7.5);
        assert_eq!(t.get(3, 0), 0.0);
    }

    #[test]
    fn argmax_and_max_work() {
        let mut t = QTable::new(1);
        t.set(0, 0, 0.1);
        t.set(0, 1, 0.2);
        t.set(0, 2, -0.5);
        assert_eq!(t.argmax_a(0), 1);
        assert_eq!(t.max_q(0), 0.2);
    }
}
```


