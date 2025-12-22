## Off‑Main‑Thread Rendering with OffscreenCanvas: A Practical Guide

### Who this is for
You have a canvas-based app (charts, games, simulations, editors) and it stutters when the UI thread is busy. You’ve heard “use `OffscreenCanvas` in a Worker,” but you want a clear explanation and a concrete path to implement it.

### What you’ll learn
- What the **main thread** does and why it gets overloaded
- What **Web Workers** are (and what they can’t do)
- What **OffscreenCanvas** enables
- A step-by-step “move rendering to a worker” plan
- Common pitfalls (input handling, resizing, device pixel ratio, Safari gaps)

---

## 1) Why the main thread is your bottleneck

In the browser, the main thread typically handles:

- DOM layout and style
- event handling (pointer/keyboard)
- JavaScript execution for UI code
- compositing and often parts of rendering

If your app renders heavy graphics or runs expensive logic on the main thread, you get:

- dropped frames
- delayed input
- janky scrolling

The goal of off-main-thread rendering is simple: **keep the UI responsive by moving expensive drawing work to a worker**.

---

## 2) Terminology

- **Main thread**: the primary JS thread that also coordinates the DOM.
- **Worker**: a background thread running JS with no direct DOM access.
- **Canvas**: a drawing surface (`<canvas>` element).
- **OffscreenCanvas**: a canvas that can be rendered to in a worker.
- **Transfer**: moving ownership of an object to another thread (so only one side controls it).

---

## 3) What OffscreenCanvas actually does

Normally, a `<canvas>` is owned by the main thread. With OffscreenCanvas you can:

1) create a canvas element in the DOM
2) call `transferControlToOffscreen()`
3) send the OffscreenCanvas to a worker via `postMessage(...)` (as a transferable)
4) perform rendering in the worker (2D or WebGL in many browsers)

Important: the worker can’t touch DOM elements; it only gets the OffscreenCanvas and any data you send.

---

## 4) Architecture: split UI from rendering

A stable approach is:

- **Main thread**
  - owns UI (DOM, React/Vue/etc.)
  - captures input events
  - sends small “input packets” to the worker
  - receives occasional state snapshots for UI (optional)

- **Render worker**
  - owns OffscreenCanvas
  - runs the render loop
  - runs simulation/scene updates (optional)

This separation makes performance predictable.

---

## 5) Worked example: move a simple animation to a worker

### Step A: main thread creates and transfers the canvas

```js
// main.js
const canvas = document.querySelector("canvas");
const offscreen = canvas.transferControlToOffscreen();

const worker = new Worker(new URL("./render.worker.js", import.meta.url), {
  type: "module",
});

worker.postMessage(
  {
    type: "INIT",
    canvas: offscreen,
    width: canvas.clientWidth,
    height: canvas.clientHeight,
    dpr: window.devicePixelRatio || 1,
  },
  [offscreen]
);
```

### Step B: worker receives the canvas and draws

```js
// render.worker.js
let ctx;
let w = 0, h = 0, dpr = 1;
let t0 = performance.now();

self.onmessage = (e) => {
  const msg = e.data;
  if (msg.type === "INIT") {
    const canvas = msg.canvas;
    w = msg.width;
    h = msg.height;
    dpr = msg.dpr;
    canvas.width = Math.floor(w * dpr);
    canvas.height = Math.floor(h * dpr);
    ctx = canvas.getContext("2d");
    requestAnimationFrame(loop);
  } else if (msg.type === "RESIZE") {
    w = msg.width; h = msg.height; dpr = msg.dpr;
    // you would also resize the OffscreenCanvas here if you stored it
  }
};

function loop(now) {
  const dt = (now - t0) / 1000;
  t0 = now;

  // Clear
  ctx.setTransform(1, 0, 0, 1, 0, 0);
  ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);
  ctx.scale(dpr, dpr);

  // Draw a moving dot
  const x = (Math.sin(now / 500) * 0.4 + 0.5) * w;
  const y = h / 2;
  ctx.beginPath();
  ctx.arc(x, y, 12, 0, Math.PI * 2);
  ctx.fillStyle = "black";
  ctx.fill();

  requestAnimationFrame(loop);
}
```

This is the smallest complete “render in a worker” example: it shows transfer, initialization, and a loop.

---

## 6) Handling input (mouse/touch/keyboard)

Workers don’t receive DOM events. The main thread must capture and forward:

- pointer position
- pointer down/up
- key down/up

**Rule**: send compact data (numbers), not whole event objects.

Example input packet:

```js
canvas.addEventListener("pointermove", (ev) => {
  worker.postMessage({ type: "POINTER", x: ev.clientX, y: ev.clientY });
});
```

---

## 7) Resizing and device pixel ratio (DPR)

The most common bug is blurry or stretched output because you forgot DPR.

Checklist:

- use `canvas.clientWidth`/`clientHeight` for layout size
- set internal buffer size to `client * dpr`
- reapply on resize and when DPR changes (zoom, moving windows between monitors)

---

## 8) Common pitfalls

- **Not transferring the OffscreenCanvas**
  - You must pass it as a transferable in `postMessage`.
- **Blocking the worker**
  - Heavy synchronous work in the render worker can still cause dropped frames.
- **Safari/compat gaps**
  - OffscreenCanvas support varies; keep a main-thread fallback.
- **Debugging difficulty**
  - Workers have separate DevTools contexts; learn how to open the worker inspector.

---

## 9) A practical “should I do this?” checklist

- Do you have a canvas scene that takes > 4–6 ms to draw per frame?
- Does UI interaction feel delayed during heavy rendering?
- Can your render loop be made mostly independent from the DOM?

If “yes,” OffscreenCanvas-in-a-worker is a strong option.


