## SoA (Structure of Arrays) vs. AoS: Data Layout for Performance

### Who this is for
You’re working on performance-sensitive code—simulation, rendering, ML, data processing—and you keep hearing: “Use SoA.” This article explains what that means, why it helps, and how to choose the right layout.

### What you’ll learn
- AoS vs. SoA in plain English
- How CPU caches and SIMD make SoA fast
- A worked example converting a struct array to SoA
- Common mistakes and a practical checklist

---

## 1) The core idea

Data layout is how you arrange your data in memory. Two common patterns:

### AoS: Array of Structs
You store a list of objects, each with all its fields together.

```text
entity[0] = {x, y, vx, vy}
entity[1] = {x, y, vx, vy}
...
```

### SoA: Structure of Arrays
You store one array per field.

```text
x[]  = [x0, x1, x2, ...]
y[]  = [y0, y1, y2, ...]
vx[] = [vx0, vx1, vx2, ...]
vy[] = [vy0, vy1, vy2, ...]
```

Both represent the same information. The performance difference comes from how CPUs read memory.

---

## 2) Why SoA is often faster

### CPU caches prefer contiguous access
If you loop over 10,000 entities and only need `x` and `y`, AoS forces the CPU to fetch `vx` and `vy` too (wasted bandwidth). SoA lets you read only what you need.

### SIMD/vectorization likes uniform arrays
Many compilers and runtimes can apply vector operations more easily when data is contiguous in one array (SoA).

### Better memory bandwidth utilization
SoA can reduce cache misses and improve throughput when your access pattern is field-centric.

---

## 3) When AoS is better

SoA is not always the winner.

AoS can be better when:
- you frequently access *all fields* together for each entity
- your code is object-centric (one entity at a time)
- you need ergonomic APIs and performance isn’t critical

Rule of thumb:
- If your hot loops look like “for all entities, update field X,” SoA usually wins.
- If your hot loops look like “for each entity, touch everything,” AoS can be fine.

---

## 4) Worked example: converting AoS → SoA

### AoS version

```js
const entities = Array.from({ length: N }, () => ({
  x: 0, y: 0, vx: 0, vy: 0
}));

for (let i = 0; i < N; i++) {
  entities[i].x += entities[i].vx * dt;
  entities[i].y += entities[i].vy * dt;
}
```

### SoA version (typed arrays)

```js
const x  = new Float32Array(N);
const y  = new Float32Array(N);
const vx = new Float32Array(N);
const vy = new Float32Array(N);

for (let i = 0; i < N; i++) {
  x[i] += vx[i] * dt;
  y[i] += vy[i] * dt;
}
```

Same math, but SoA makes memory access predictable and compact.

---

## 5) Common mistakes

- **Mixing layouts accidentally**
  - You keep AoS objects but also maintain SoA arrays → you pay twice.
- **Over-optimizing cold code**
  - Convert only the parts that are truly in the hot path.
- **Ignoring alignment/stride when interoperating**
  - When data is shared across boundaries (GPU, WASM, network), define a stable schema.

---

## 6) A practical checklist

- What is your hot loop reading/writing?
- Is access field-centric (SoA) or entity-centric (AoS)?
- Can you represent data with typed arrays for predictable layout?
- Can you batch operations to maximize contiguous access?
- Have you measured before/after with a real profiler?


