import { defineConfig } from "vite";
import path from "node:path";

export default defineConfig({
  // GitHub Pages serves project sites under `/<repo>/`.
  // We set this in CI via BASE_URL, but keep "/" for local dev.
  base: process.env.BASE_URL || "/",
  server: {
    port: 5173,
    strictPort: true,
    fs: {
      // Allow importing the wasm-pack output from `../pkg/` (experiment root).
      allow: [path.resolve(__dirname, "..")]
    }
  }
});


