# Persistence with OPFS

Training becomes meaningful when you can keep what you learned.

DrillSergeantHQ targets a practical persistence story:

- autosave checkpoints locally
- export a model file for sharing
- import a model file to continue training or to demo behavior

## Why OPFS (Origin Private File System)?

OPFS behaves like a real filesystem inside the browser:

- fast read/write for binary blobs
- better suited to large files than LocalStorage
- often simpler ergonomics than managing IndexedDB records directly

In the technical design, OPFS is the preferred backend for storing weight blobs.

## File format philosophy

Keep it boring and debuggable:

- a small header with:
  - magic bytes / version
  - shape hash / architecture id
  - byte length
- a body containing raw `Float32` weights (contiguous)

This format works well with the SAB double-buffer contract:

- you can write the entire blob quickly
- you can verify shape/version before loading

## Autosave vs export

- **Autosave**: periodic (e.g., every 5 minutes) and on “Stop Training”
- **Export**: user-triggered download of a model file (e.g., `.dshq`)

## Failure modes

- **Shape mismatch on import**:  
  - defense: shape hash check; require compatible architecture or a migration tool

- **Partial writes** (crash mid-save):  
  - defense: write to temp file then rename/commit (transaction-ish pattern)

Persistence is not glamorous, but it’s the difference between a demo and a tool people actually use.

---

**Prev:** [Part VII — Storage and Deployment](README.md)  
**Next:** [COOP/COEP and Cross-Origin Isolation](cross-origin-isolation.md)


