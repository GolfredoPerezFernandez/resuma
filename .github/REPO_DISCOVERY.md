# Repository discoverability (GitHub & search)

This file documents how the **Resuma Rust web framework** is labeled so it is not confused with unrelated “resume/CV builder” repos named `resuma`.

## GitHub Settings (owner)

1. **About → Description** — set via `gh repo edit` or UI; must mention *Rust*, *SSR*, *web framework*.
2. **About → Website** — https://resuma-docs.fly.dev
3. **Topics** — `rust`, `web-framework`, `ssr`, `axum`, `resumability`, etc.
4. **Social preview** — upload `assets/social-preview.png` (export from `assets/social-preview.svg`) under **Settings → General → Social preview**.

## Crates.io

Keywords and `description` in the workspace `Cargo.toml` apply on the **next** `cargo publish`.

## Docs site

Canonical product URL: https://resuma-docs.fly.dev — links back to this repo in JSON-LD and README.
