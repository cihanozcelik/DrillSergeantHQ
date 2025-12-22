# Buffer Layouts (Draft)

This appendix is where “conceptual contracts” become concrete offsets and strides.

It is intentionally marked **draft** because exact observation/action schemas may evolve. The goal is to lock down format once the core shapes stabilize.

## Weights publishing (double buffer)

### Buffers

- `weights_A`: `Float32Array` (SAB)
- `weights_B`: `Float32Array` (SAB)
- `weights_ctrl`: `Int32Array` (SAB, Atomics)

### `weights_ctrl` (Int32 indices)

| Index | Name | Type | Writer | Reader |
|---:|---|---|---|---|
| 0 | `active_idx` | i32 | Trainer | Render/Eval |
| 1 | `version` | i32 | Trainer | Render/Eval |
| 2 | `shape_hash` | i32 | Trainer | Render/Eval |
| 3 | `step_counter` | i32 | Trainer | Render/Eval/UI |

## Rollout transport (ring buffer)

Because N→1 multi-producer rings are complex, the recommended v1 design is **per-producer lanes**:

- each rollout worker owns a 1→1 ring buffer
- trainer polls/consumes from all rings

### Per-producer ring control (conceptual)

| Field | Type | Meaning |
|---|---|---|
| `head` | i32 | next write index |
| `tail` | i32 | next read index |
| `capacity` | i32 | number of chunks |
| `dropped` | i32 | drop counter |

### Chunk payload (suggested)

SoA payload for one rollout window:

- `obs[t][env]` (float32)
- `act[t][env]` (int32)
- `rew[t][env]` (float32)
- `done[t][env]` (int32 or u8 packed)
- `logp[t][env]` (float32)
- `value[t][env]` (float32)

Once the observation vector dimension and action encoding are finalized, this appendix should specify:

- exact byte offsets for each array
- stride formulas
- alignment requirements

---

**Prev:** [Message Schemas (Draft)](message-schemas.md)  
**Next:** [Math Cheat Sheet](math-cheat-sheet.md)


