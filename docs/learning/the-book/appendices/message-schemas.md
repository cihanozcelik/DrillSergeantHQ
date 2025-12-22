# Message Schemas (Draft)

These schemas are “draft” because exact fields will evolve, but the discipline should not: message passing is for **control** and **telemetry**, not bulk data.

## UI → Render/Eval Worker

### `INIT_CANVAS`

- **purpose**: transfer OffscreenCanvas and initialize rendering
- **fields**:
  - `offscreen`: OffscreenCanvas (transferable)
  - `w`, `h`: physical canvas dimensions
  - `dpi`: device pixel ratio

### `SET_MODE`

- **fields**:
  - `mode`: `"ai_vs_ai" | "human_vs_ai" | "sandbox"`

### `INPUT_BATCH`

- **fields**:
  - `events[]`: pointer/key events (timestamped)

### `SHOW_CONFIG`

- **fields**:
  - `action_repeat`: int
  - `swap_interval_ms`: int
  - `blend_ms`: int (optional)

### `PLAY` / `PAUSE` / `RESET_SHOW`

- `RESET_SHOW` optionally accepts `seed`

### `SET_SIM_SPEED`

- **fields**:
  - `multiplier`: float (e.g., 0.25, 1, 2, 4)

## UI → Trainer Worker

### `START_TRAINING`

- **fields (representative)**:
  - `seed`
  - `num_envs`
  - `T` (rollout horizon)
  - `gamma`, `lambda`
  - `clip` (epsilon)
  - `lr`
  - `epochs`
  - `minibatch`

### `STOP_TRAINING`

### `SET_HYPER`

- **fields**:
  - `key`, `value`

### `RESET_TRAINER`

## Trainer → UI

### `METRICS` (1–2 Hz)

- **fields (representative)**:
  - losses: `loss_policy`, `loss_value`, `loss_entropy`
  - performance: `sps`, `ups`
  - outcomes: `avg_reward`, `win_rate_ema`, `episode_length`
  - hyperparams: `current_lr`, `clip_epsilon`

---

**Prev:** [System Diagrams (Text + Mermaid)](diagrams.md)  
**Next:** [Buffer Layouts (Draft)](buffer-layouts.md)


