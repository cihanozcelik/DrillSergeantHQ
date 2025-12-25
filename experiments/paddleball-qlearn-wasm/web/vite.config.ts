import { defineConfig, type Plugin } from "vite";
import { spawn } from "node:child_process";
import path from "node:path";

export default defineConfig({
  plugins: [wasmPackPlugin()],
  server: {
    port: 5173,
    strictPort: true,
    // We watch Rust sources from the sibling `../rust` directory.
    fs: { allow: [".."] }
  }
});

function wasmPackPlugin(): Plugin {
  const rustDir = path.resolve(__dirname, "..", "rust");
  const outDir = path.resolve(__dirname, "src", "pkg");

  let building = false;
  let queued = false;

  async function build(server?: import("vite").ViteDevServer) {
    if (building) {
      queued = true;
      return;
    }
    building = true;

    const args = ["build", "--dev", "--target", "web", "--out-dir", outDir];

    await new Promise<void>((resolve) => {
      const p = spawn("wasm-pack", args, {
        cwd: rustDir,
        stdio: "inherit",
        shell: process.platform === "win32"
      });
      p.on("close", () => resolve());
      p.on("error", () => resolve());
    });

    // Force reload so updates in generated pkg are applied reliably.
    if (server) server.ws.send({ type: "full-reload" });

    building = false;
    if (queued) {
      queued = false;
      await build(server);
    }
  }

  return {
    name: "wasm-pack-watch",
    apply: "serve",
    async configureServer(server) {
      // Initial build before first page load.
      await build(server);

      // Watch Rust sources + Cargo files.
      const watchGlobs = [
        path.resolve(rustDir, "src", "**/*"),
        path.resolve(rustDir, "Cargo.toml"),
        path.resolve(rustDir, "Cargo.lock")
      ];
      server.watcher.add(watchGlobs);
      server.watcher.on("change", async (file) => {
        if (file.startsWith(rustDir)) await build(server);
      });
    }
  };
}


