# Resuma bundle benchmark

Measure initial JavaScript for a resumable counter page.

## Quick check

```bash
# Embedded asset sizes (raw + gzip + brotli)
cd runtime && npm run build && npm run size

# Live JSON from a running server
curl -s http://127.0.0.1:3000/_resuma/benchmark.json

# Interactive counter example
cargo run -p example-counter
```

With gzip enabled:

```bash
curl -s -H "Accept-Encoding: gzip" -I http://127.0.0.1:3000/_resuma/loader.js
curl -s -H "Accept-Encoding: br" -I http://127.0.0.1:3000/_resuma/core.js
```

## Methodology

1. Same UX: SSR heading + one interactive counter button.
2. Chrome DevTools → Network, disable cache, hard reload.
3. Compare **transfer size** with gzip/brotli enabled (production server).
4. Report raw (uncompressed) and compressed bytes separately.

## Resuma (split runtime)

| Bundle | When loaded | Raw | Gzip | Brotli |
|--------|-------------|-----|------|--------|
| `loader.js` | Interactive pages only | ~1.8 KiB | ~884 B | ~730 B |
| `core.js` | First interaction or reactive bindings | ~6.6 KiB | ~2.6 KiB | ~2.3 KiB |
| Static docs page | Never | 0 B | 0 B | 0 B |

## Takeaways

- **Static-first:** Resuma skips loader, payload, and runtime on pages with no interactivity.
- **Small loader:** `loader.js` is under 1 KiB gzip on a typical counter page.
- **Honest totals:** Full interactivity still loads `core.js` — report loader + core, not just the loader.
- **Production:** Asset routes serve gzip/brotli based on `Accept-Encoding`.
