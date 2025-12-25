# Experimentation checklist (beginner-friendly)

This file is intentionally practical: tweak one knob, observe behavior, take notes.

## Rendering knobs (Phase 0)

- Canvas size (smaller = faster)
- Paddle/ball colors
- Coordinate mapping (NDC vs pixel)

## Physics knobs (Phase 1)

- `dt` (fixed timestep): try `1/30`, `1/60`, `1/120`
- Ball speed:
  - initial `vx`, `vy`
  - max speed clamp
- Restitution:
  - wall bounce coefficient
  - paddle bounce coefficient
- Paddle:
  - width/height
  - **max speed**

## Reward knobs (Phase 2)

- Sparse only:
  - `+1` bounce
  - `-1` terminal
- (Later) shaped reward variants:
  - small + for keeping ball above paddle line
  - small - for time

## Discretization knobs (Phase 3)

- Bin counts:
  - ball x bins
  - ball y bins
  - vx bins
  - vy bins
  - paddle x bins
- Value ranges per dimension (clamp ranges)

## Q-learning knobs (Phase 3)

- `epsilon` (exploration): start ~`0.2`, decay to `0.05`
- `alpha` (learning rate): start ~`0.1`
- `gamma` (discount): start ~`0.99`
- Episode reset conditions
- Steps per render frame (training speed)

## What to record (recommended)

- Average episode length over last N episodes
- Average reward over last N episodes
- Catch rate
- Epsilon schedule and whether learning is stable


