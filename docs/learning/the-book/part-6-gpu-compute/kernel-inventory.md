# Kernel Inventory for a PPO Learner

Think of the PPO learner as a sequence of operations on large tensors.

The exact network can change, but the *class* of kernels needed is stable. This chapter lists those kernels so the reader can see the system as a pipeline, not a mystery.

## Dataflow at a high level

```text
obs batch
  ↓
MLP forward
  ↓
policy logits + value
  ↓
logprob/entropy + value loss + PPO surrogate
  ↓
backward pass (gradients)
  ↓
Adam update
```

## Kernel categories (from the design)

### Linear algebra

- `linear_forward`: \(Y = XW + b\)
- `linear_backward`: gradients for \(W\), \(b\), and \(X\)

### Activations

- `activation_forward`: ReLU/Tanh
- `activation_backward`

### Policy/value heads

- head forward for logits
- head forward for value

### Probability math

- softmax (or log-softmax) for logits
- `logprob(a_t)` for sampled actions
- `entropy` for exploration monitoring/regularization

### Reductions and normalization

- advantage normalization (mean/std)
- loss reductions over batch

### PPO loss

- compute ratio
- clip ratio
- compute surrogate objective

### Optimizer

- `adam_update` (parameter update)

## Buffer layout philosophy

For throughput:

- use contiguous buffers
- keep SoA-style organization (batch-major)
- minimize intermediate allocations by reusing scratch buffers

## The reason for a kernel inventory

It’s easy to “add compute until it works.”

It’s harder (and more valuable) to design a learner like an engine subsystem:

- explicit stages
- explicit buffers
- explicit correctness checks

The next chapter is about correctness: validation mode.

---

**Prev:** [Where WebGPU Is Used](where-webgpu-is-used.md)  
**Next:** [Validation Mode: CPU Reference Paths](validation-mode.md)


