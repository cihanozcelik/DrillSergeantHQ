# Step 22 — Save/Load Q-table + doc polish

Goal: make learning persistent and make the experiment approachable.

## 22-01 Implement save/load (localStorage is fine) with metadata
- **What**: save a JSON blob containing:
  - bins config
  - num_states
  - q values
  - version string
- **Why**: prevents loading mismatched tables after you change discretization.

## 22-02 Add UI buttons: Save, Load, Clear Save
- **What**: small HTML panel (or simple buttons in `web/src/main.ts`).
- **Why**: newbies shouldn’t need devtools for persistence.

## 22-03 Validate mismatch (don’t load if config differs; show message)
- **What**: compare bins + num_states + version before loading.
- **Why**: avoids silent corruption.

## 22-04 Update `README.md` with user controls + what “good learning” looks like
- **What**: explain modes (Train/Pause/Eval), how to save/load, what improvement looks like.
- **Why**: sets expectations and reduces confusion.

## Code (suggested approach)

### Data format (Rust side)

You can serialize to JSON manually (no extra deps) by exporting the Q-table as a comma-separated string, or you can add `serde` (depends on repo policy). If you want **no dependencies**, do this:

```rust
pub fn qtable_to_csv(q: &[f32]) -> String {
    let mut s = String::new();
    for (i, v) in q.iter().enumerate() {
        if i > 0 { s.push(','); }
        s.push_str(&format!("{v}"));
    }
    s
}

pub fn qtable_from_csv(s: &str) -> Vec<f32> {
    s.split(',')
        .filter_map(|x| x.parse::<f32>().ok())
        .collect()
}
```

### localStorage (JS side)

```ts
type SaveBlob = {
  version: string;
  bins: { ball_x: number; ball_y: number; ball_vx: number; ball_vy: number; paddle_x: number; vmax: number };
  numStates: number;
  qCsv: string;
};

localStorage.setItem("paddleball_q", JSON.stringify(blob));
const raw = localStorage.getItem("paddleball_q");
```

### Validation

- If `blob.version !== expectedVersion` → show message, refuse load
- If `blob.numStates !== currentNumStates` → refuse load
- If bins differ → refuse load


