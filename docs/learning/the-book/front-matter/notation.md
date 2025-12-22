# Notation and Conventions

## System terms

- **Show environment / show match**: the single environment instance the user watches (and may interact with).
- **Training environments**: headless batched environments used to produce rollouts for learning.
- **Render/Eval worker**: worker hosting the show env and its render loop.
- **Trainer worker**: worker hosting PPO updates and checkpoint publishing.
- **Rollout workers**: N workers that run batched environments and write experiences into shared memory.

## Symbols

- \(s_t\): observation/state at time \(t\)
- \(a_t\): action chosen at time \(t\)
- \(r_t\): reward at time \(t\)
- \(\gamma\): discount factor
- \(\lambda\): GAE parameter
- \(V(s)\): value function
- \(\pi(a|s)\): policy distribution

## Code and pseudo-code

This book includes pseudo-code for:

- **SharedArrayBuffer contracts** (atomic indices, ring buffers)
- **Fixed timestep simulation**
- **PPO/GAE loops**

Pseudo-code aims to express the *contract* and *shape* of the solution. The real project may differ in naming and exact layout, but should preserve the same invariants.

## “Contract boxes”

When you see a boxed section labeled **Contract**, read it as an API boundary. These are the things you coordinate across files, languages (TS/Rust), and threads/workers.

---

**Prev:** [How to Use This Book](how-to-use-this-book.md)  
**Next:** [Part I — The Product and the Constraints](../part-1-product-and-constraints/README.md)


