# Protocols: Messages and Contracts

In DrillSergeantHQ, message passing is not how we “stream the world.” Message passing is how we **issue commands** and **report health**.

If you send big blobs over `postMessage`, you will eventually rebuild a slow, fragile, copy-heavy architecture. Instead:

- **Messages** are for *intent* (play/pause, change mode, start training).
- **Shared memory** is for *bulk data* (weights, rollouts).

This chapter defines the mindset and the types of messages we allow.

## What messages are for

### Control (UI → workers)

Examples:

- start/stop training
- play/pause/reset show match
- change sim speed
- update hyperparameters
- update rendering/show configuration

These messages should be:

- small (bytes, not megabytes)
- infrequent (human rate, not frame rate)
- idempotent where possible

### Telemetry (Trainer → UI)

Telemetry is intentionally slow:

- 1–2 Hz is usually enough for human monitoring
- higher rates can be requested in debug builds, but default should be calm

Telemetry answers “is training alive?” and “is it improving?” without drowning the UI.

## What messages are not for

- streaming weights every checkpoint (use SAB double buffer)
- streaming rollouts (use SAB ring buffer)
- driving per-frame rendering state (the show worker owns its state)

If you see a design that “needs” huge messages, it’s usually a sign a contract is missing.

## Contract: message shape stability

Every message type is an API surface. Treat it like one.

**Contract**

- Every message has a `type` field.
- Every message schema is versionable (explicit or implicit).
- Unknown message types are safely ignored (or logged) in dev mode.
- Messages never contain raw pointers or platform-specific objects except in initialization (e.g., OffscreenCanvas transfer).

## A minimal message taxonomy (from the technical design)

### UI → Render/Eval Worker

- `INIT_CANVAS { offscreen, w, h, dpi }`
- `SET_MODE { ai_vs_ai | human_vs_ai | sandbox }`
- `INPUT_BATCH { events[] }`
- `SHOW_CONFIG { action_repeat, swap_interval_ms, blend_ms }`
- `PLAY` / `PAUSE` / `RESET_SHOW { seed? }`
- `SET_SIM_SPEED { multiplier }`

### UI → Trainer Worker

- `START_TRAINING { seed, num_envs, T, gamma, lambda, clip, lr, epochs, minibatch, ... }`
- `STOP_TRAINING`
- `SET_HYPER { key, value }`
- `RESET_TRAINER`

### Trainer → UI (telemetry)

At 1–2 Hz:

- `METRICS { loss_policy, loss_value, loss_entropy, sps, ups, avg_reward, win_rate_ema, ... }`

## Why “protocol discipline” matters

The fastest way to lose control of a multi-worker system is to let protocols evolve implicitly.

Discipline gives you:

- clear ownership boundaries
- reproducible bugs (because the same inputs mean the same behavior)
- the ability to upgrade subsystems without “everything must change at once”

Protocols are the “engine headers” of DrillSergeantHQ. Treat them with respect.

---

**Prev:** [Timing and Loops: Show vs Training](timing-and-loops.md)  
**Next:** [Shared Memory Strategy (SAB First)](shared-memory-strategy.md)


