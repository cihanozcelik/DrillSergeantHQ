# Hot-Swapping Policies Without Breaking Reality

Hot-swapping is where DrillSergeantHQ becomes *itself*.

Plenty of systems can train an agent. Fewer can let you watch a match while training runs, then inject improved behavior without a restart.

This chapter explains how to make hot-swapping feel safe and natural.

## What hot-swapping is (and is not)

Hot-swapping means:

- keep the same simulation state (positions, velocities, cooldowns, RNG state)
- replace the policy parameters used for action selection

Hot-swapping does **not** mean:

- teleporting entities
- resetting the world
- mixing user interventions into training data

## The mechanical core: pointer swap + version check

From Part III, weights are published via a double buffer:

- trainer writes the inactive buffer
- trainer flips `active_idx` and increments `version`
- show worker detects version change and switches weight pointer

This is the whole trick: **no big messages**, no copies, no blocking.

## The human problem: avoiding “snap”

Even when the mechanics are correct, the behavior change can feel abrupt.

There are three strategies, and DrillSergeantHQ can support all of them:

1. **Swap on reset only**  
   - safest for perception, slowest feedback  
2. **Swap mid-session** (default)  
   - fastest feedback, can look “snappy” early in training  
3. **Swap with blending**  
   - mix old/new policies for a short window (advanced)

### Blending concept (advanced)

During a blend window, sample actions from:

\[
\pi_{blend} = (1-\alpha)\pi_{old} + \alpha \pi_{new}
\]

where \(\alpha\) ramps from 0→1 over, say, 0.5–2 seconds.

This is optional. The baseline v1 system works without it.

## A hot-swap schedule that feels good

Human perception likes regularity. A sane default is:

- publish checkpoints at ~1 Hz
- hot-swap on version change (checked on a timer)

Too frequent swaps can create the impression of “unstable AI,” even if learning is fine.

## Failure modes

- **Swap while reading partially written weights**:  
  - defense: double buffer + publish discipline (write then flip)

- **Architecture mismatch** (new weights expect different model shape):  
  - defense: `shape_hash` check; if mismatch, refuse swap and require reset/reinit

- **Perceived regression** (new policy is temporarily worse):  
  - defense: show mode can use opponent pool snapshots, or swap less frequently, or blend

Hot-swapping is an *experience feature*. The engineering must serve the experience.

---

**Prev:** [Action Selection and “No Teleport Jumps”](action-selection.md)  
**Next:** [Part V — Training System](../part-5-training-system/README.md)


