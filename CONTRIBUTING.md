# Contributing

Thanks for your interest in DrillSergeantHQ.

## What this repo is (and isn’t)

- This is a **from-scratch** learning project focused on **fundamentals**: simulation, inference, training loops, memory layouts, and GPU kernels.
- We intentionally avoid “black box” engines/frameworks that replace the core work.

## Quick start (for contributors)

- Keep PRs small and focused.
- Prefer performance-oriented, allocation-aware code.
- Add minimal docs/comments when introducing new concepts (especially anything ABI / buffer layout related).

## Before you open a PR

- Open an issue (or start a short discussion) for any change that affects:
  - shared memory layouts / ABI
  - protocol messages between workers
  - training loop semantics
  - determinism guarantees

## Style / expectations

- **Determinism** matters for simulation and tests.
- **Copy-free data paths** are preferred where possible (SAB/ring buffers).
- Keep dependencies minimal; “protocol glue” deps are fine, full engines/ML stacks are not.

## Docs

- Main technical design lives in `docs/technical-design.md`.


