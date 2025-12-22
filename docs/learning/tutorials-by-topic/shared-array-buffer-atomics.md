## SharedArrayBuffer and Atomics: Shared Memory Concurrency in JavaScript

### Who this is for
You want real parallelism in the browser: multiple Workers cooperating on the same data without copying megabytes back and forth. You’ve heard of `SharedArrayBuffer` and `Atomics`, but the terminology and failure modes are confusing.

### What you’ll learn
- What `SharedArrayBuffer` is (and how it differs from `ArrayBuffer`)
- What `Atomics` does and why you need it
- The basic patterns: **flags**, **counters**, **ring buffers**
- A concrete example: a producer/consumer queue
- The gotchas: memory ordering, busy-waiting, and security requirements

---

## 1) The problem: postMessage copies (or it used to)

Workers communicate via `postMessage()`. For large data, you have two options:

- **Copy** data (slow for large buffers)
- **Transfer** ownership (fast, but only one side can use the buffer)

`SharedArrayBuffer` enables a third option:

- **Share** the same memory between threads (fast and concurrent)

---

## 2) What is SharedArrayBuffer?

`SharedArrayBuffer` (SAB) is a block of memory that can be accessed by:

- the main thread
- one or more workers

Unlike `ArrayBuffer`, it is not “owned” by a single thread. Multiple threads can read and write it.

Because that can be dangerous without coordination, SAB is typically paired with:

- **typed arrays** (e.g., `Int32Array`, `Float32Array`) for views
- **Atomics** for safe synchronization on integer views

---

## 3) Why Atomics exists

If two threads write to the same variable at the same time, you can get races:

- lost updates (two increments become one)
- reading “half-written” values
- inconsistent state (one thread sees the first half of an update)

`Atomics` provides operations that are:

- atomic (happen as one indivisible step)
- ordered (with rules that prevent reordering surprises)

Important constraint:

- `Atomics` works on **integer typed arrays** (commonly `Int32Array`) backed by a `SharedArrayBuffer`.

---

## 4) Core patterns you’ll use

### A) A flag (“is data ready?”)
- Writer sets `flag = 1` after writing data.
- Reader waits for `flag == 1`, then reads data.

### B) A counter (“how many items?”)
- Use `Atomics.add()` to increment safely.

### C) Ring buffer (queue)
- One or more producers push records
- One consumer pops records
- Use head/tail indices stored in an atomic control block

---

## 5) Worked example: single-producer / single-consumer ring buffer

We’ll build a tiny fixed-capacity queue for integers. Real systems store structured records, but the control logic is similar.

### Memory layout

- `data`: `Int32Array` of length `CAP`
- `ctrl`: `Int32Array` of length 2
  - `ctrl[0] = head` (write index)
  - `ctrl[1] = tail` (read index)

The queue is empty when `head == tail`.

### Producer (push)

```js
function push(data, ctrl, value) {
  const CAP = data.length;
  const head = Atomics.load(ctrl, 0);
  const tail = Atomics.load(ctrl, 1);
  const next = (head + 1) % CAP;

  if (next === tail) return false; // full

  data[head] = value;
  Atomics.store(ctrl, 0, next);
  return true;
}
```

### Consumer (pop)

```js
function pop(data, ctrl) {
  const CAP = data.length;
  const head = Atomics.load(ctrl, 0);
  const tail = Atomics.load(ctrl, 1);

  if (tail === head) return null; // empty

  const value = data[tail];
  Atomics.store(ctrl, 1, (tail + 1) % CAP);
  return value;
}
```

This is the basic idea: store payload in `data`, store coordination in `ctrl`.

---

## 6) Waiting efficiently: Atomics.wait / Atomics.notify

Busy-wait loops (“spin until head changes”) waste CPU. On platforms that support it, you can use:

- `Atomics.wait(typedArray, index, expectedValue)` in a worker
- `Atomics.notify(typedArray, index, count)` to wake sleepers

Pattern:

- consumer waits when queue empty
- producer notifies after pushing

This turns “polling” into “sleep until work arrives.”

---

## 7) Big gotchas (read this before shipping)

### Cross-origin isolation requirement
In many browsers, SAB requires the page to be cross-origin isolated (COOP/COEP). If SAB is missing, check:

- `crossOriginIsolated`
- response headers

### Memory ordering complexity
Atomics provides ordering guarantees, but mixing atomic and non-atomic writes can still be tricky. A safe approach:

- write payload first
- then `Atomics.store()` the flag/index that “publishes” the payload

### Multi-producer queues are harder
If multiple producers push concurrently, you need stronger coordination (CAS loops, per-producer regions, or separate queues).

---

## 8) A practical adoption checklist

- Do you truly need shared memory (throughput) vs. messages (simplicity)?
- Can you define a stable memory layout (offsets/strides) and version it?
- Do you have a plan for debugging races (logging, invariants, checksums)?
- Do you have a fallback path if SAB is unavailable?


