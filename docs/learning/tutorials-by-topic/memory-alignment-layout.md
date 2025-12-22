## Memory Alignment and Data Layout: The Part That Makes Fast Code Actually Work

### Who this is for
You’re moving data between systems—CPU ↔ GPU, JS ↔ WASM, one thread ↔ another—and you keep encountering bugs that look like “random corruption.” Often, it’s not random: it’s **alignment, padding, and layout mismatches**.

### What you’ll learn
- What **alignment** is and why it exists
- What **padding** is and how it sneaks into structs
- How to design a stable **binary layout** (offsets/strides)
- How to share data across languages safely (an ABI contract)
- A practical checklist for avoiding “it works on my machine” layout bugs

---

## 1) Alignment in plain English

Computers read memory in chunks. Many CPUs and GPUs are faster (or require) when certain values start at addresses that are multiples of 2, 4, 8, 16 bytes, etc.

**Alignment** is that rule: “this type should start at an address divisible by N.”

Examples:
- A 32-bit integer (`u32`) often wants 4-byte alignment.
- A 4-float vector (`vec4<f32>`) often wants 16-byte alignment in GPU layouts.

When you violate alignment rules you might get:
- slower loads
- validation errors
- wrong values (especially in GPU/shader interfaces)

---

## 2) Padding: the invisible bytes

When you define a struct, the compiler may insert **padding bytes** so each field is properly aligned.

Example conceptually:

```text
struct {
  u8   a;    // 1 byte
  // 3 bytes padding here so next field aligns to 4
  u32  b;    // 4 bytes
}
```

The struct is not 5 bytes—it’s commonly 8 bytes because of padding.

This is where cross-language bugs come from: different languages or compilers can lay out fields differently unless you specify rules explicitly.

---

## 3) Offsets, stride, and “binary contracts”

When you store many records, two numbers matter:

- **offset**: byte position of a field from the start of the record
- **stride**: byte size of one record (how far to jump to reach the next)

If you and your consumer disagree on either, you read garbage.

### A tiny example contract (AoS)

You decide a record is:

- `x`: float32 at offset 0
- `y`: float32 at offset 4
- `vx`: float32 at offset 8
- `vy`: float32 at offset 12
- stride = 16 bytes

Now any language can read it by the same offsets.

---

## 4) GPU layouts: why 16-byte alignment shows up a lot

Many GPU APIs and shader languages use layout rules that strongly prefer (or require) 16-byte alignment for certain structures, especially vectors and arrays.

Practical rule:

- If you’re designing data for the GPU, default to **16-byte multiples** for stride where possible.

Even when the GPU can read unaligned data, alignment often improves performance and reduces surprises.

---

## 5) Sharing data across JS/WASM/Workers: define one ABI

If you have a binary buffer shared between components (for example, between a worker and a main thread, or JS and WASM), treat the layout as an **ABI** (Application Binary Interface) contract:

- version the layout
- define offsets and sizes
- keep an explicit schema in documentation

### Recommended practice

- Store a small header:
  - magic bytes (file/format identifier)
  - version number
  - record size / stride
  - counts
  - optional checksum

Then store the raw data with the agreed layout.

---

## 6) SoA vs AoS and alignment

SoA often simplifies alignment because each field is a dedicated typed array:

- `Float32Array` is naturally aligned for 4-byte floats
- data is contiguous and easy to vectorize

AoS can be fine, but requires careful stride management and consistent packing rules.

---

## 7) Common failure modes (symptom → cause)

- **Values look “shifted”**
  - cause: different stride/offset assumptions
- **Only some fields are wrong**
  - cause: padding differences, misaligned vector fields
- **Works in debug, fails in release**
  - cause: compiler/layout differences, missing “repr” / packing constraints
- **GPU validation errors**
  - cause: binding layout doesn’t match shader expectations

---

## 8) A practical checklist

- **Write down the layout**
  - offsets, stride, types
- **Choose a layout style**
  - SoA for throughput; AoS for ergonomics
- **Add a versioned header**
  - detect mismatches early
- **Test with known patterns**
  - write 0,1,2,3… and verify reads match expectations
- **Keep alignment conservative**
  - especially for GPU-facing data

