# Telemetry: Making Training Observable

Training is a control system. Without instruments, you don’t have a system—you have a rumor.

DrillSergeantHQ’s product promise (“watch learning live”) depends on telemetry for two reasons:

- the UI needs to show that training is alive and progressing
- developers need to catch failure modes early (collapse, divergence, stalls)

Telemetry is intentionally slow (1–2 Hz) and high signal.

## What to report (minimum viable dashboard)

### Losses (health of the update)

- `loss_policy`
- `loss_value`
- `loss_entropy`

These let you detect:

- policy collapse (entropy → 0)
- value divergence (value loss spikes / NaNs)

### Performance (is the machine doing work?)

- `steps_per_second` (SPS)
- `updates_per_second` (UPS) or `seconds_per_update`
- optional: GPU time estimate

This answers “are we rollout-bound or update-bound?”

### Environment outcomes (is behavior improving?)

- `avg_reward` (with decomposition if possible)
- `win_rate_ema`
- `episode_length`

### Hyperparameters (debugging context)

- current learning rate
- clip epsilon
- entropy coefficient

## Contract: telemetry must be comparable over time

If metrics are computed inconsistently, trends become meaningless.

**Contract**

- report units explicitly (per second, per update, per episode)
- use EMA windows with documented smoothing factors
- include counters (total steps, total updates) so plots can align

## Debugging “hidden state”

Two non-negotiables from the design:

- **CPU reference path** for GPU kernels (validation mode)
- `wgpu` validation layers in development builds

Telemetry should surface “validation mismatch” warnings, not bury them in logs.

## Failure modes (what telemetry catches)

- **Training dead**: SPS/UPS near 0  
  - causes: worker crashed, ring buffer stuck, SAB not available

- **Divergence**: NaNs, losses exploding  
  - causes: learning rate too high, reward scale, kernel bug, GAE bug

- **Slow death**: entropy decays, reward plateaus  
  - causes: exploration too low, reward too sparse, opponent too strong

Telemetry is how we turn “it feels wrong” into “here’s the graph that explains it.”

---

**Prev:** [Self-Play and Stabilization](self-play.md)  
**Next:** [Part VI — GPU Compute](../part-6-gpu-compute/README.md)


