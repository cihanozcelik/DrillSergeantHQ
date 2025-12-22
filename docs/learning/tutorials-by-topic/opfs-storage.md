## OPFS (Origin Private File System): Fast Local Storage in the Browser

### Who this is for
You’re building a web app that needs to store larger files locally—models, projects, media, caches, exports—and `localStorage` or IndexedDB feels slow, awkward, or too limited. You’ve heard about OPFS and want to understand what it is and how to use it.

### What you’ll learn
- What OPFS is (and how it differs from other storage)
- Key terminology: **origin**, **sandbox**, **handles**, **streams**
- How to read/write files using OPFS
- When OPFS is a good idea (and when it isn’t)
- Practical safety: quotas, user expectations, backups

---

## 1) What is OPFS?

**OPFS** stands for **Origin Private File System**. It’s a browser-provided filesystem that belongs to your site’s **origin** (your domain + scheme + port).

You can think of OPFS as:

- a private folder the browser gives to your web app
- with file and directory semantics (create, read, write, rename)
- designed for performance and large-ish data

Unlike “download a file” or “ask the user to pick a folder,” OPFS is typically **silent** and **app-managed**: the app decides what to store and how.

---

## 2) Why OPFS exists (the pain it solves)

Traditional browser storage options have tradeoffs:

- **localStorage**: tiny and synchronous (can block UI), not for files
- **IndexedDB**: powerful but awkward for large blobs and file-like updates
- **Cache API**: good for HTTP caching, not general app files

OPFS targets a common need: **fast file I/O for web apps that behave like desktop apps**.

---

## 3) Terminology

- **Origin**: `https://example.com` (plus port). Each origin gets its own OPFS.
- **DirectoryHandle**: a reference to a directory.
- **FileHandle**: a reference to a file.
- **WritableStream**: a stream you write bytes to.
- **Quota**: how much storage the browser will allow your origin to use.

---

## 4) The basic workflow: open directory → open file → read/write

Modern browsers expose OPFS via the Storage Foundation / File System Access style APIs. The most common pattern is:

1) Get the origin’s root directory.
2) Create or open a file.
3) Write bytes to it (often via a writer/stream).
4) Later, read the file.

### Example: write a JSON file (high-level pattern)

```js
// Pseudocode-ish: exact names can vary by browser implementation.
const root = await navigator.storage.getDirectory();
const fileHandle = await root.getFileHandle("settings.json", { create: true });

const writable = await fileHandle.createWritable();
await writable.write(JSON.stringify({ theme: "dark" }));
await writable.close();
```

### Example: read it back

```js
const root = await navigator.storage.getDirectory();
const fileHandle = await root.getFileHandle("settings.json");
const file = await fileHandle.getFile();
const text = await file.text();
const obj = JSON.parse(text);
```

These examples illustrate the mental model: **OPFS is file handles + async reads/writes**.

---

## 5) Writing large binary data (the “real use case”)

If you’re storing large data (e.g., binary blobs), treat it like a file:

- write in chunks
- avoid rewriting the entire file if you only update a small segment (if your format allows)
- keep metadata (version, checksum) in a small header file

Common patterns:

- `model.bin` (binary weights)
- `model.json` (metadata like shapes, version, date)
- `autosave/` directory for snapshots

---

## 6) What about performance?

OPFS is typically faster for file-style workflows than shoving everything into IndexedDB records, especially when:

- files are large
- you rewrite often
- you want sequential reads/writes

Still, performance depends on browser and device. Measure:

- write time (ms)
- read time (ms)
- file size
- frequency of saves

---

## 7) Practical product concerns (don’t skip these)

### Quota and eviction
Browsers enforce storage limits. If you store a lot, you need a plan:

- show the user how much space you’re using
- provide “delete old projects” or “clear cache”
- keep exports available (download) so users can back up

### User expectations
Users expect:

- autosave is real
- there is an export/import story
- “clear site data” can wipe everything

### Backups
OPFS is local to one browser profile. If users care about the data, give them:

- export to file (download)
- import from file

---

## 8) A checklist for adopting OPFS

- Do you need **file semantics** (directories, names, large sequential reads/writes)?
- Do you need **better performance** than IndexedDB for large blobs?
- Can you provide **export/import** and a “clear storage” UX?
- Are you prepared to handle **quota limits**?

If yes, OPFS is a strong choice for serious web apps.


