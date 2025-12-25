# Step 14 — Deterministic RNG + seeding (reproducible learning)

Goal: add a tiny RNG so “randomness” is **reproducible** (same seed → same behavior).

## 14-01 Implement a tiny RNG in pure Rust (LCG/xorshift)
- **What to add**: `rust/src/rng.rs`
- **Technique**: use xorshift64* (tiny, decent quality for this use).
- **Why**: Rust `std` has no RNG; we want no extra deps and deterministic WASM/native behavior.

## 14-02 Add a seed and log it in the overlay
- **What**: store `seed: u64` in `World` or in the training controller.
- **Technique**: start with a fixed seed constant so newbies can reproduce runs.
- **Why**: debugging learning without reproducibility is painful.

## 14-03 Randomize initial ball velocity slightly on reset (bounded)
- **What**: in `World::reset_episode()`, pick small random vx/vy around defaults.
- **Technique**: sample `[-1,1]` and scale; keep speed bounded.
- **Why**: avoids the agent overfitting a single trajectory.

## 14-04 Verify same seed produces same first N episodes
- **What**: write a unit test that runs N resets and compares results.
- **Why**: proves determinism.

## Code

### New file: `rust/src/rng.rs` (full file)

```rust
#[derive(Clone, Copy, Debug)]
pub struct Rng {
    state: u64,
}

impl Rng {
    pub fn new(seed: u64) -> Self {
        // avoid zero lock-up for xorshift
        let seed = if seed == 0 { 0x9E3779B97F4A7C15 } else { seed };
        Self { state: seed }
    }

    pub fn next_u64(&mut self) -> u64 {
        // xorshift64*
        let mut x = self.state;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.state = x;
        x.wrapping_mul(0x2545F4914F6CDD1D)
    }

    pub fn next_f32_0_1(&mut self) -> f32 {
        // Use top 24 bits for a float in [0,1)
        let v = (self.next_u64() >> 40) as u32; // 24 bits
        (v as f32) / ((1u32 << 24) as f32)
    }

    pub fn next_f32_neg1_1(&mut self) -> f32 {
        self.next_f32_0_1() * 2.0 - 1.0
    }
}
```

### Wire it into the crate: `rust/src/lib.rs`

```rust
mod rng;
```

### Use it in `rust/src/world.rs` (example)

Add fields:

```rust
pub rng: crate::rng::Rng,
pub seed: u64,
```

Initialize in `World::new()`:

```rust
let seed = 12345u64;
let rng = crate::rng::Rng::new(seed);
```

Randomize in `reset_episode()` (bounded):

```rust
let dvx = self.rng.next_f32_neg1_1() * 0.10; // ±0.10
let dvy = self.rng.next_f32_neg1_1() * 0.10;
self.ball.vx = (0.20 + dvx).clamp(-0.6, 0.6);
self.ball.vy = (0.15 + dvy).clamp(-0.6, 0.6);
```


