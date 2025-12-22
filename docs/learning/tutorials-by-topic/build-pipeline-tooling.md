## Build Pipelines and Tooling: How Modern Web Projects Actually Ship

### Who this is for
You’re building a web project that’s more than “one JS bundle”—maybe it includes TypeScript, Web Workers, and WebAssembly. You want a mental model for the build pipeline and a practical checklist for making it reliable in production.

### What you’ll learn
- What a **build pipeline** is (and why it’s more than “run the bundler”)
- How bundling changes when you add **Workers**
- How **WASM** fits into modern toolchains
- Why **headers** (like COOP/COEP) can be a build-and-deploy concern
- A practical troubleshooting playbook

---

## 1) What is a build pipeline?

A build pipeline is the end-to-end process that converts your source code into deployable artifacts:

- TypeScript/JS → browser-ready JS
- CSS preprocessors → CSS
- assets → optimized + fingerprinted files
- optional: Rust/C/C++ → WASM

It’s not just compilation. It also includes:

- bundling
- code splitting
- asset copying
- environment configuration
- production caching rules

If your “pipeline” ends at `npm run build`, you’re missing the part where real deployments fail.

---

## 2) The four stages you should design explicitly

### Stage A: Authoring (developer experience)
- Fast local iteration (hot reload)
- Clear module boundaries
- Shared types/schemas where needed

### Stage B: Compilation (language → JS/WASM)
- TypeScript transpilation (or typecheck + transpile)
- WASM compilation (if used)

### Stage C: Bundling (turn modules into deployable entrypoints)
- Main app bundle(s)
- One bundle per Worker entry
- Chunking for shared dependencies

### Stage D: Serving/Deployment (the often-forgotten stage)
- Correct MIME types
- Correct headers
- Correct caching
- Correct base paths and asset URLs

---

## 3) Workers change bundling: treat each Worker as a small app

A Web Worker has its own global scope, dependency graph, and runtime constraints.

Practical consequences:

- A worker needs its own **entry file**.
- Worker imports must be resolvable by the bundler.
- Anything loaded by the worker (scripts, WASM, assets) must be present in the output and served correctly.

### Common Worker pitfalls

- **“Worker script not found”** in production
  - Cause: the bundler emitted the worker file in a different path than expected.
  - Fix: use a bundler-supported worker URL pattern and verify output.

- **Worker loads in dev but fails in prod**
  - Cause: base path differences or caching stale worker code.
  - Fix: fingerprint worker bundles and avoid aggressive caching for HTML.

---

## 4) WASM in the pipeline: two integration patterns

### Pattern 1: WASM built as an external artifact
You run a dedicated WASM build step that produces:

- a `.wasm` binary
- optional JS “glue” code for loading/exports

Then your bundler copies these into the final build.

**Why teams like this**: clear separation and caching.  
**Why teams dislike this**: two pipelines to keep aligned.

### Pattern 2: WASM built via a bundler plugin
The bundler triggers the WASM compilation and treats outputs like modules.

**Why teams like this**: one command, smoother dev loop.  
**Why teams dislike this**: plugin quirks; debugging can be opaque.

---

## 5) Headers as a “build concern”: COOP/COEP (and similar constraints)

Some advanced browser features require specific security headers. That’s not “just ops”—it affects whether your app works at all.

Example: enabling cross-origin isolation requires:

- `Cross-Origin-Opener-Policy: same-origin`
- `Cross-Origin-Embedder-Policy: require-corp`

Practical implications:

- Your dev server must set these headers so you can test locally.
- Your production host/CDN must also set them.
- Any cross-origin embedded resources must be compatible with these policies.

Even if your code is perfect, missing headers can disable key APIs.

---

## 6) The production output you should aim for

You want an output directory where everything is explicit and cacheable:

```text
dist/
  index.html
  assets/
    app.<hash>.js
    styles.<hash>.css
    worker-a.<hash>.js
    worker-b.<hash>.js
    module.<hash>.wasm
```

Rules:

- Every runtime-loaded file gets a **content hash**.
- `index.html` is cached lightly (or not cached), so it can reference new hashes.

---

## 7) Troubleshooting playbook (what to check first)

- **Worker fails to start**
  - Check: emitted worker file exists in the build output
  - Check: `type: "module"` if you rely on ES module imports
  - Check: correct path/base URL in production

- **WASM fails to load**
  - Check: `.wasm` exists in output
  - Check: server serves `.wasm` with correct MIME type (`application/wasm`)
  - Check: fetch path is correct inside worker/main thread

- **Feature is “missing” in production**
  - Check: required headers are present (COOP/COEP, CSP, etc.)
  - Check: `crossOriginIsolated` (when relevant)

- **Works in dev, breaks after deploy**
  - Check: caching (stale HTML referencing missing hashed assets)
  - Check: CDN rewrite rules and base path

---

## 8) A shipping checklist

- **Entrypoints**
  - Main app builds
  - Every worker builds as its own entry
- **Artifacts**
  - WASM files included and fingerprinted (if used)
  - Assets are copied and referenced correctly
- **Serving**
  - Correct MIME types (especially `.wasm`)
  - Required headers configured
- **Caching**
  - Hashed assets cached long
  - HTML cached short

---

## Further reading

- Tooling docs for your bundler (Vite/webpack/Rollup/esbuild) on Workers and WASM
- Hosting docs for setting headers and caching on your target platform (CDN, static host, reverse proxy)

