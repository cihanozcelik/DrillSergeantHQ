# 🏗️ PaddleBall Scaffold (Newbie Explainer)
## A guided tour of the current code + the full per-frame flow

This document explains the **current** PaddleBall scaffold in a newbie-friendly way:
- What runs where (TypeScript vs Rust/WASM vs GPU shader)
- The exact “baton pass” control flow
- How a single frame becomes pixels
- Where to change things (with copy-find anchors)
- Tiny experiments to prove you understand it

This scaffold is intentionally minimal: it draws **one paddle + one ball** and handles resize correctly.

**Important**: the learning experiment built on top of this scaffold is **not deep learning**. It uses **tabular Q-learning** (a Q-table), not a neural network.

---

## Run it first (so you know your baseline works)

From repo root:

```bash
cd experiments/paddleball-qlearn-wasm
npm install
npm run dev
```

Open the URL printed by Vite (usually `http://localhost:5173/`).

**Expected**:
- A letterboxed canvas
- A **teal** paddle (rectangle)
- An **orange** ball (circle)
- **Walls** on the top/left/right edges

If you don’t see that, stop here and fix the environment first (WebGPU must be available in your browser).

---

## 0) The 60-second mental model

If you remember only one thing, remember this:

```text
TypeScript owns the canvas size and starts WASM
Rust owns the render loop and uploads a small struct (uniforms) every frame
The GPU shader draws the paddle + ball purely from that uniform data
```

### The “baton pass” flow (open the page → first frame)

```text
web/src/main.ts
  - create <canvas>
  - compute CSS size + pixel backing size (DPR-aware)
  - await init()  (loads WASM)
  - run(canvas)   (hand canvas to Rust)

rust/src/lib.rs
  - run() installs panic/log hooks
  - spawn_local(async { render::run_canvas(canvas).await })

rust/src/render.rs
  - RenderState::new(): WebGPU setup + pipeline + uniform buffer
  - start_animation_loop(): requestAnimationFrame → update() + render()

rust/src/shader.wgsl
  - vs_main draws a fullscreen triangle
  - fs_main runs on every pixel and decides its color (paddle/ball/bg)
```

---

## 1) Mini glossary (only the terms this project uses)

- **CSS size**: `canvas.style.width/height` (how big the canvas looks)
- **Backing store size**: `canvas.width/height` (how many pixels we render)
- **DPR**: `devicePixelRatio` (Retina screens have DPR > 1)
- **Surface**: WebGPU’s presentation target (here: the canvas)
- **Adapter**: “which GPU am I using?”
- **Device**: creates GPU resources (buffers, pipelines)
- **Queue**: uploads data and submits GPU work
- **Uniform buffer**: small read-only “settings” data for shaders each frame
- **Bind group**: “connect this buffer to this shader binding”
- **NDC**: normalized device coords in [-1..1]
- **UV**: normalized coords in [0..1]
- **SDF**: signed distance field math (how we draw shapes without meshes)

---

## 2) TypeScript (`web/src/main.ts`): canvas sizing + starting WASM

### 2.1 What you should search for

In `web/src/main.ts`, search for:
- `contentAspect`
- `canvas.width =`
- `await init()`
- `run(canvas)`

### 2.2 Why “CSS size vs backing store size” matters

Beginners often do this wrong and get **blurry** rendering.

The project intentionally separates:
- **CSS size** (layout): `canvas.style.width/height`
- **Backing store size** (pixels): `canvas.width/height`

Annotated excerpt (resize + DPR):

```ts
// web/src/main.ts
const dpr = window.devicePixelRatio || 1;
canvas.style.width = `${cssW}px`;
canvas.style.height = `${cssH}px`;

const desiredW = Math.max(1, Math.floor(cssW * dpr));
const desiredH = Math.max(1, Math.floor(cssH * dpr));
```

**If you get this wrong**:
- You’ll see blur on Retina screens.

### 2.3 Resize handshake: TS → WASM flag

When TS changes backing store size, it also notifies Rust:

```ts
// web/src/main.ts
canvas.width = p.w;
canvas.height = p.h;
wasm_notify_resize?.();
wasm_set_dpr?.(dpr);
```

**Why the notify call exists**: Rust needs to reconfigure the WebGPU surface, and the easiest cross-boundary signal is a tiny WASM export that flips a flag.

### 2.4 WASM startup

This is the “start everything” moment:

```ts
// web/src/main.ts
await init();
run(canvas);
```

`init()` loads the wasm-pack bundle, and `run(canvas)` calls the Rust-exported entrypoint.

---

## 3) Rust entry (`rust/src/lib.rs`): the WASM-exported `run(canvas)`

### 3.1 What you should search for

In `rust/src/lib.rs`, search for:
- `#[wasm_bindgen]`
- `pub fn run(`
- `spawn_local`

### 3.2 What happens in `run(canvas)`

Annotated excerpt:

```rust
// rust/src/lib.rs
#[wasm_bindgen]
pub fn run(canvas: HtmlCanvasElement) {
    console_error_panic_hook::set_once();
    let _ = console_log::init_with_level(log::Level::Info);

    wasm_bindgen_futures::spawn_local(async move {
        if let Err(err) = render::run_canvas(canvas).await {
            log::error!("fatal: {err:?}");
        }
    });
}
```

**What this means**:
- TS calls `run(canvas)` once.
- Rust starts an async task.
- From this point, Rust owns the render loop.

---

## 4) WASM bridge (`rust/src/wasm_api.rs`): resize flags

### 4.1 The “flag” pattern (why it exists)

JS can fire resize events at any time. Rust’s renderer wants to *check* whether it should resize during its own frame loop. So we store a flag.

Annotated excerpt:

```rust
// rust/src/wasm_api.rs
thread_local! {
    static NEEDS_RESIZE: Cell<bool> = Cell::new(false);
}

#[wasm_bindgen]
pub fn wasm_notify_resize() {
    NEEDS_RESIZE.with(|v| v.set(true));
}

pub fn take_needs_resize() -> bool {
    NEEDS_RESIZE.with(|v| {
        let cur = v.get();
        if cur { v.set(false); }
        cur
    })
}
```

**Edge-trigger behavior**: `take_needs_resize()` returns true once, and clears the flag.

---

## 5) Renderer (`rust/src/render.rs`): the per-frame pipeline

### 5.1 The two biggest ideas

- **One struct** (`SceneUniforms`) is the CPU→GPU “contract” each frame.
- Each frame is:

```text
update()  -> edit uniforms + upload them (queue.write_buffer)
render()  -> draw fullscreen triangle (shader uses uniforms to color pixels)
```

### 5.2 `SceneUniforms`: the CPU→GPU contract

Search for `struct SceneUniforms`.

Important facts:
- `#[repr(C)]` ensures predictable memory layout (GPU needs this).
- `bytemuck::Pod` allows `bytes_of(&uniforms)` safely.

If this struct and the WGSL `struct Scene` ever disagree (field order/types), you’ll get wrong shapes/colors or nothing rendered.

### 5.3 WebGPU setup: Instance → Surface → Adapter → Device/Queue

This is the “boot” sequence in `RenderState::new(canvas)`:

```rust
let instance = wgpu::Instance::default();
let surface = instance.create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone()))?;
let adapter = instance.request_adapter(...).await.ok_or_else(...)?;
let (device, queue) = adapter.request_device(...).await?;
```

**Newbie translation**:
- Instance: entrypoint to WebGPU
- Surface: “where to present” (the canvas)
- Adapter: which GPU backend to use
- Device: create buffers/pipelines
- Queue: send work and upload data

### 5.4 Uniform buffer upload (the most important line)

Search for `write_buffer(`.

```rust
// rust/src/render.rs
self.queue
    .write_buffer(&self.uniforms_buffer, 0, bytemuck::bytes_of(&self.uniforms));
```

**What this does**:
- Takes the CPU struct `self.uniforms`
- Turns it into bytes
- Copies it into GPU memory (`uniforms_buffer`)

If you forget to call this, the shader will keep reading old values and nothing changes visually.

### 5.5 Render pass + draw (why `draw(0..3, 0..1)` works)

Search for `begin_render_pass` and `draw(0..3`.

```rust
rpass.set_pipeline(&self.pipeline);
rpass.set_bind_group(0, &self.uniforms_bind_group, &[]);
rpass.draw(0..3, 0..1);
```

**Newbie translation**:
- “Use this shader program” (`set_pipeline`)
- “Here is the uniform buffer for @group(0) @binding(0)” (`set_bind_group`)
- “Draw 3 vertices” (`draw(0..3, ...)`) → vertex shader generates a fullscreen triangle

### 5.6 The render loop: requestAnimationFrame

Search for `request_animation_frame`.

The closure does:
- `s.update()`
- `s.render()`
- schedules itself again

This is why the animation runs continuously.

---

## 6) Shader (`rust/src/shader.wgsl`): how pixels are decided

### 6.1 Uniform binding (GPU sees the data)

Search for `@group(0) @binding(0)`.

```wgsl
@group(0) @binding(0)
var<uniform> scene: Scene;
```

This must match the bind group Rust created for the uniform buffer.

### 6.2 Vertex shader: fullscreen triangle

Search for `vs_main`.

The positions:
- (-1, -3), (3, 1), (-1, 1)

Cover the whole screen. The point is: *we don’t draw a paddle mesh; we draw one big triangle so `fs_main` runs on every pixel*.

### 6.3 Fragment shader: UV space + SDF math

Search for `fs_main`.

Key steps:
- Convert NDC [-1..1] → UV [0..1]
- Compute distance to paddle rectangle and ball circle
- Color pixels inside the shapes

---

## 7) Try-it-now experiments (fast feedback)

### Experiment A: prove the shader is running

File: `rust/src/shader.wgsl`

Change:
- `let paddle_col = vec3<f32>(0.30, 0.85, 0.75);`
to something obvious like bright red.

**Expected**: paddle color changes immediately after refresh.

### Experiment B: prove CPU→GPU uniform upload

File: `rust/src/render.rs`, in `fn update(&mut self)`

Temporarily move the ball:
- set `self.uniforms.ball_x = 0.2;`
- set `self.uniforms.ball_y = 0.8;`

**Expected**: ball jumps to the new position.

If nothing changes, you edited the wrong function OR the app isn’t rebuilding as you expect.

### Experiment C: prove the resize handshake

File: `web/src/main.ts`

Temporarily comment out:
- `wasm_notify_resize?.();`

Resize the window.

**Expected**: you may see weirdness during resize (surface not reconfigured when TS changes backing store).

Undo afterwards.

---

## 8) Debugging checklist (common newbie failures)

### Black / empty screen

- Is WebGPU available? (`"gpu" in navigator` must be true)
- Open the browser console:
  - Rust panics/logs should appear (because of `console_error_panic_hook` + `console_log`)
- If you see `No suitable GPU adapter found`, adapter selection failed.
- If you see repeated `render error: ...`, the surface might be lost or misconfigured.

### Blurry output

- Backing store must be `floor(css * dpr)` (TS does this).
- If you changed sizing code, ensure `canvas.width/height` matches DPR.

### Shapes stretch / circle looks like an oval

- `uniforms.aspect` must be updated on resize (`resize_if_needed()` does this).
- Shader uses `scene.aspect` to correct X distances; if aspect is wrong, circles become ovals.

---

## 9) Why this scaffold exists (and what you change next)

This scaffold is “correct” because:
- The **render loop is owned by Rust** (good for later simulation/RL in Rust).
- The **CPU→GPU boundary is explicit** (`SceneUniforms` + uniform buffer upload).
- Resize is handled with a clean **TS→WASM flag → Rust surface reconfigure** handshake.
- The shader is minimal and deterministic (fullscreen triangle + SDF).
