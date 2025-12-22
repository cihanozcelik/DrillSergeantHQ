## COOP/COEP and Cross‑Origin Isolation: Why Your Web App Suddenly Needs “Special Headers”

### Who this is for
You’re building a modern web app and you run into a frustrating message like:

- “`SharedArrayBuffer` is not defined”
- “`crossOriginIsolated` is false”
- “This page is not cross-origin isolated”

You’re told to add COOP/COEP headers—but what are they, why do they exist, and what will break when you enable them? This article answers those questions and gives you a practical deployment checklist.

### What you’ll learn
- What **cross-origin isolation** means
- What **COOP** and **COEP** headers do (in plain English)
- How to enable them safely
- The common breakages (CDNs, iframes, third-party scripts, images)
- Debugging steps and a shipping checklist

---

## 1) The problem COOP/COEP solves

Browsers enforce strong isolation between websites for security. Some advanced features—especially ones that enable high-performance shared memory—raise the stakes for side-channel attacks.

To reduce risk, browsers require an explicit opt-in: **cross-origin isolation**.

When a page is cross-origin isolated, the browser can safely enable certain capabilities (depending on the browser), such as:

- `SharedArrayBuffer` (shared memory between threads)
- high-performance WASM threading patterns
- more predictable isolation guarantees between tabs/windows

---

## 2) Key terminology

- **Origin**: the tuple `(scheme, host, port)`. `https://example.com:443` is a different origin from `http://example.com` or `https://cdn.example.com`.
- **Same-origin**: two URLs share the same origin.
- **Cross-origin**: different origin.
- **Cross-origin isolated**: the page has opted into a stricter isolation mode enforced by the browser.

In JavaScript you can often check:

```js
console.log("crossOriginIsolated:", crossOriginIsolated);
```

---

## 3) The two headers: COOP and COEP

### COOP: Cross-Origin-Opener-Policy
**COOP controls how your page “opens” and interacts with other browsing contexts** (tabs/windows).

The common setting:

```text
Cross-Origin-Opener-Policy: same-origin
```

Effect (high-level):
- Your page is put into a separate “group” so it doesn’t share certain resources with cross-origin pages.

### COEP: Cross-Origin-Embedder-Policy
**COEP controls what your page is allowed to embed** (images, scripts, iframes, etc.) unless they explicitly allow it.

The common setting:

```text
Cross-Origin-Embedder-Policy: require-corp
```

Effect (high-level):
- Your page can only load cross-origin resources if they explicitly opt in (via CORS or CORP headers).

---

## 4) Enabling cross-origin isolation (the practical recipe)

To enable cross-origin isolation for a typical web app:

1) Serve your HTML and JS with:

```text
Cross-Origin-Opener-Policy: same-origin
Cross-Origin-Embedder-Policy: require-corp
```

2) Ensure every cross-origin resource you load is compatible:
- Same-origin resources are generally fine.
- Cross-origin resources must be served with appropriate headers (CORS or CORP) so they are embeddable under `require-corp`.

---

## 5) What breaks when you turn this on

This is the part everyone learns the hard way.

### Third-party scripts (analytics, widgets)
Many third-party scripts load more subresources (images, iframes, wasm) from their own origins without the headers you need. Result: resources get blocked.

**Fixes:**
- Host critical scripts yourself (same-origin).
- Choose vendors that support COEP-compatible loading.
- Or don’t enable COEP on pages that must embed arbitrary third-party resources.

### Images/fonts from CDNs
If your CSS loads fonts from a CDN, those font files may be blocked.

**Fixes:**
- Serve fonts/images from the same origin.
- Configure the CDN to send compatible headers.

### Iframes
Embedding cross-origin iframes gets harder.

**Fixes:**
- If you control the iframe origin, configure it properly.
- If you don’t control it, you may need to redesign the integration.

---

## 6) Debugging checklist

When `crossOriginIsolated` is false:

- **Check headers are present on the main document** (HTML response).
- **Check headers are present on the app shell** (your JS bundles).
- **Open DevTools → Network** and inspect the response headers.
- **Look for blocked resources** in Console/Network.
  - If something is blocked due to COEP, fix that resource first.

When a single blocked resource exists, it can prevent cross-origin isolation from becoming active.

---

## 7) A safe rollout strategy

If your product has pages with many third-party embeds, don’t flip COEP everywhere at once.

Safer patterns:

- Enable COOP/COEP only on routes that need `SharedArrayBuffer`/threading.
- Keep marketing pages and embed-heavy pages separate.
- Validate in staging with production-like CDN config (caching + headers).

---

## 8) Shipping checklist (copy/paste)

- **Headers**
  - `Cross-Origin-Opener-Policy: same-origin`
  - `Cross-Origin-Embedder-Policy: require-corp`
- **Verification**
  - `crossOriginIsolated === true` in production
  - `SharedArrayBuffer` is available where you expect
- **Assets**
  - Fonts/images/scripts are either same-origin or explicitly embeddable
- **Third parties**
  - Analytics/widgets verified under COEP (or excluded from isolated pages)
- **Docs**
  - Team knows which pages require isolation and why


