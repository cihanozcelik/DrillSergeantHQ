import init, { run } from "./pkg/paddleball_qlearn_wasm.js";

function setupCanvas(): HTMLCanvasElement {
  const app = document.getElementById("app");
  if (!app) throw new Error("Missing #app root");

  const canvas = document.createElement("canvas");
  canvas.id = "game";
  app.appendChild(canvas);

  const resize = () => {
    // Important: set actual pixel size (not just CSS size).
    const dpr = window.devicePixelRatio || 1;
    canvas.width = Math.max(1, Math.floor(window.innerWidth * dpr));
    canvas.height = Math.max(1, Math.floor(window.innerHeight * dpr));
  };

  window.addEventListener("resize", resize);
  resize();

  return canvas;
}

async function main() {
  if (!("gpu" in navigator)) {
    const msg =
      "WebGPU is not available. Try Chrome/Edge and ensure WebGPU is enabled.";
    document.body.innerHTML = `<pre style="color:#eee;padding:16px">${msg}</pre>`;
    return;
  }

  const canvas = setupCanvas();
  await init();
  // Rust owns the render loop; this just hands over the canvas.
  run(canvas);
}

main().catch((err) => {
  // eslint-disable-next-line no-console
  console.error(err);
});


