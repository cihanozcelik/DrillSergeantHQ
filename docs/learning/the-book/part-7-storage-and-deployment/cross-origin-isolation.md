# COOP/COEP and Cross-Origin Isolation

If DrillSergeantHQ is an engine, then **cross-origin isolation** is the “graphics driver requirement.”

Without it, you lose:

- SharedArrayBuffer
- WASM threads (in the browser security model)

And without those, the project’s performance architecture collapses.

## What “cross-origin isolated” means

In the browser, cross-origin isolation is a security posture enabled by HTTP headers.

The technical design’s required headers are:

- `Cross-Origin-Opener-Policy: same-origin`
- `Cross-Origin-Embedder-Policy: require-corp`

When configured correctly, `crossOriginIsolated === true` in the page.

## Why the project depends on it

High-throughput paths depend on SAB:

- rollout ring buffers
- weights double buffers

WASM threads typically rely on:

- shared memory
- atomic operations
- worker-based thread pools

## Deployment implications (practical)

Cross-origin isolation affects:

- dev server configuration (must send headers)
- hosting/CDN configuration (headers must be present on HTML and often on subresources)
- asset loading policy (`require-corp` can block cross-origin resources without CORP headers)

The system architecture assumes these constraints are handled early—because otherwise “max performance” isn’t reachable.

## A deployment checklist

- verify `crossOriginIsolated === true`
- verify workers can be created successfully
- verify SAB-backed typed arrays initialize
- verify WebGPU is available (or activate fallback strategy)

## Failure modes

- **SAB unavailable**:  
  - symptom: shared memory init fails, or training is forced to slow paths
  - cause: missing/incorrect COOP/COEP headers

- **Resources blocked**:  
  - symptom: fonts/images/scripts fail to load under `require-corp`
  - cause: cross-origin assets without proper CORP/CORS headers

Cross-origin isolation is “boring infrastructure,” but it is the foundation for the entire worker + SAB architecture.

---

**Prev:** [Persistence with OPFS](opfs.md)  
**Next:** [Appendices](../appendices/README.md)


