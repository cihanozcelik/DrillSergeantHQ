import { defineConfig } from "vite";
import path from "node:path";

export default defineConfig({
  server: {
    port: 5173,
    strictPort: true,
    fs: {
      // Allow importing the wasm-pack output from `../pkg/` (experiment root).
      allow: [path.resolve(__dirname, "..")]
    }
  }
});


