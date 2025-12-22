## `requestAnimationFrame` in Dedicated Workers: Building Smooth Worker Loops

### Who this is for
You’re running rendering or simulation logic in a Dedicated Worker and you want a smooth frame loop similar to the main thread’s `requestAnimationFrame` (rAF). This tutorial explains how to structure worker loops, when rAF exists, and how to build a safe fallback.

### What you’ll learn
- What rAF is (and what it guarantees)
- How a worker loop differs from a main-thread loop
- The “render tick” vs “simulation tick” separation
- Fallback strategies (`setTimeout`, fixed timestep)
- A checklist for robust worker loops

---

## 1) What is `requestAnimationFrame`?

`requestAnimationFrame(callback)` schedules `callback` to run before the next repaint. It’s designed for visual updates:

- good for smooth animations
- helps the browser coordinate timing
- provides a timestamp

On the main thread, rAF is the standard animation loop tool.

---

## 2) Can workers use rAF?

In some browsers/environments, Dedicated Workers provide a form of `requestAnimationFrame`. In others, you may not have it—or it may behave differently than on the main thread.

Practical engineering rule:
> Feature-detect and provide a fallback.

Don’t build your system such that “worker rAF must exist” unless you control the runtime environment.

---

## 3) Architecture: separate simulation rate from render rate

Even if you have rAF, you often want:

- **Render loop**: runs at display refresh rate (e.g., ~60Hz)
- **Simulation loop**: runs at a fixed dt (e.g., 120Hz) for determinism/stability

The standard approach uses an accumulator:

```text
on frame tick:
  accumulator += elapsed_time
  while accumulator >= dt:
    step_simulation(dt)
    accumulator -= dt
  render()
```

Render ticks can be driven by rAF; simulation ticks are fixed.

---

## 4) A robust worker loop pattern (feature detect)

Pseudo-logic:

```js
const raf = self.requestAnimationFrame?.bind(self);

function startLoop() {
  if (raf) raf(frame);
  else setTimeout(() => frame(performance.now()), 16);
}

let last = performance.now();
let acc = 0;
const dt = 1 / 120;

function frame(now) {
  const elapsed = Math.min(0.25, (now - last) / 1000); // clamp spikes
  last = now;
  acc += elapsed;

  while (acc >= dt) {
    stepSimulation(dt);
    acc -= dt;
  }

  render();
  startLoop();
}
```

This gives you:
- smooth render ticks when available
- deterministic simulation stepping
- safety against huge time spikes

---

## 5) Common pitfalls

- **Spiral of death**
  - If simulation can’t keep up, the while-loop runs forever.
  - Fix: clamp max steps per frame or drop excess accumulated time.
- **Variable dt simulation**
  - Leads to nondeterminism and instability.
  - Fix: fixed dt accumulator loop.
- **Too much per-frame work**
  - Even in workers, heavy work can cause jank and input lag.
  - Fix: profiling and budgeting; move heavy work to compute passes or separate workers.

---

## 6) Checklist

- Feature-detect worker rAF; fallback exists
- Simulation uses fixed timestep; render uses frame tick
- Time spike clamping prevents explosions after tab sleep
- Max simulation steps per frame prevents spiral-of-death
- Loop can be started/stopped cleanly

