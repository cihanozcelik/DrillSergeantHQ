import init, {
  run,
  wasm_notify_resize,
  wasm_set_dpr
} from "../../pkg/paddleball_qlearn_wasm.js";

function setupCanvas(): { canvas: HTMLCanvasElement; container: HTMLElement } {
  const app = document.getElementById("app");
  if (!app) throw new Error("Missing #app root");

  // Center the canvas and allow letterboxing.
  app.style.display = "flex";
  app.style.alignItems = "center";
  app.style.justifyContent = "center";

  const canvas = document.createElement("canvas");
  canvas.id = "game";
  app.appendChild(canvas);

  return { canvas, container: app };
}

async function main() {
  if (!("gpu" in navigator)) {
    const msg =
      "WebGPU is not available. Try Chrome/Edge and ensure WebGPU is enabled.";
    document.body.innerHTML = `<pre style="color:#eee;padding:16px">${msg}</pre>`;
    return;
  }

  const { canvas, container } = setupCanvas();

  // --- MagCreate-style resize handling ---
  let wasmReady = false;
  let resizeRaf: number | null = null;
  let pendingBacking: { w: number; h: number } | null = null;
  let lastApplyMs = 0;

  // Keep a stable aspect ratio to avoid stretching visuals.
  // (MagCreate preserves content aspect via camera/content sizing.)
  const contentAspect = 16 / 9;

  const resizeCanvas = () => {
    const dpr = window.devicePixelRatio || 1;
    const rect = container.getBoundingClientRect();

    // Letterbox: choose the largest canvas that fits the container while preserving aspect.
    let cssW = rect.width;
    let cssH = rect.height;
    if (cssW > 0 && cssH > 0) {
      const fitH = cssW / contentAspect;
      if (fitH <= cssH) {
        cssH = fitH;
      } else {
        cssW = cssH * contentAspect;
      }
    }

    canvas.style.width = `${cssW}px`;
    canvas.style.height = `${cssH}px`;

    // Backing size should match the CSS size (not the container size),
    // otherwise the content will be stretched.
    const desiredW = Math.max(1, Math.floor(cssW * dpr));
    const desiredH = Math.max(1, Math.floor(cssH * dpr));

    // Before WASM starts, set backing size immediately so init sees correct dimensions.
    if (!wasmReady) {
      canvas.width = desiredW;
      canvas.height = desiredH;
      return;
    }

    pendingBacking = { w: desiredW, h: desiredH };
    if (resizeRaf != null) return;

    resizeRaf = requestAnimationFrame(() => {
      resizeRaf = null;
      const p = pendingBacking;
      pendingBacking = null;
      if (!p) return;

      // Optional throttle (MagCreate style): avoid spamming surface reconfigure during live resize.
      const now = performance.now();
      if (now - lastApplyMs < 33) {
        pendingBacking = p;
        resizeCanvas();
        return;
      }
      lastApplyMs = now;

      if (canvas.width !== p.w || canvas.height !== p.h) {
        canvas.width = p.w;
        canvas.height = p.h;
        try {
          wasm_notify_resize?.();
          wasm_set_dpr?.(dpr);
        } catch {
          // ignore
        }
      }
    });
  };

  resizeCanvas();
  window.addEventListener("resize", resizeCanvas);
  const ro = new ResizeObserver(() => resizeCanvas());
  ro.observe(container);

  // Wait until canvas has non-zero size before starting WASM.
  const startWhenReady = async () => {
    if (canvas.width === 0 || canvas.height === 0) {
      requestAnimationFrame(() => void startWhenReady());
      return;
    }

    await init();
    wasmReady = true;
    // seed DPR into WASM (optional but keeps things consistent)
    try {
      wasm_set_dpr?.(window.devicePixelRatio || 1);
    } catch {
      // ignore
    }

    // Rust owns the render loop; this just hands over the canvas.
    run(canvas);
  };

  await startWhenReady();
}

main().catch((err) => {
  // eslint-disable-next-line no-console
  console.error(err);
});


