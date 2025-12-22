# Preface

Most ML systems are explained backwards.

They start with an algorithm and end with a user experience. But DrillSergeantHQ exists to deliver a very specific feeling:

> You can watch a match, press **Train**, and see behavior improve while you’re still watching.

That promise dictates everything. It dictates why training is isolated from the show match. It dictates why policies are hot‑swapped without restarting. It dictates why we prefer shared memory over message passing. It dictates why we obsess over determinism and validation.

This book treats DrillSergeantHQ the way a renderer book treats a frame: as a pipeline with strict budgets, contracts, and invariants. You’ll see the system as a set of collaborating runtimes (threads/workers), data channels (messages/shared buffers), and loops (fixed timestep, PPO updates), all designed to feel *continuous* and *observable*.

If you’re the kind of person who wants to remove “magic boxes” by rebuilding the stack one layer down—welcome.

---

**Prev:** [Title Page](title-page.md)  
**Next:** [How to Use This Book](how-to-use-this-book.md)


