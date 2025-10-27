<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

# Leptos Pokédex

A client-side rendered Leptos app that fetches Pokémon data from the PokéAPI and displays animated cards styled after the mockups in `design.html`. GitHub Pages deployment is handled via the included workflow.

## Prerequisites

- Rust toolchain (see `rust-toolchain.toml` for the pinned version)
- `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- [Trunk](https://trunkrs.dev/) for local dev/builds: `cargo install trunk`
- (Optional) `npm install -g pnpm` if you want to experiment with frontend tooling alongside the Rust app

## Client-Side Rendering with Trunk

This repo ships as a client-side only (CSR) Leptos app, ideal for static WASM hosting or wrappers like Tauri.

Install the Trunk CLI and the WebAssembly target once:

```bash
cargo install --locked trunk
rustup target add wasm32-unknown-unknown
```

Then launch the live-reload dev server from the project root:

```bash
trunk serve --open
```

Trunk bundles the CSR build, watches `src/`, `style.css`, and `index.html`, and opens `http://127.0.0.1:8080/` by default.

## Run Locally

```bash
trunk serve --open
```

Trunk compiles the Leptos app to WebAssembly, watches for changes in `src/`, `style.css`, and `index.html`, and opens a dev server at `http://127.0.0.1:8080/` by default.

If you change dependencies or the Rust toolchain, rebuild once with `trunk build` so the new artifacts are cached before serving again.

## Build for GitHub Pages

```bash
trunk build --release --public-url ./
```

The optimized bundle is emitted to `dist/` and mirrors what the GitHub Actions workflow deploys. Inspect `dist/` locally or push to `main` to let CI publish automatically.

## Deployment via GitHub Pages

1. In repository settings, open **Pages** and set the source to **GitHub Actions**.
2. Ensure `.github/workflows/gh-pages-deploy.yml` exists in your repo (copy it here if starting fresh).
3. Push or merge to `main`; the workflow runs Trunk and publishes the `dist/` folder to Pages.

## Troubleshooting

- `cargo check` validates the Leptos code without compiling WASM.
- `cargo fmt && cargo clippy --all-targets --all-features` keeps formatting and linting tidy before commits.
- When the dev server fails to refresh, purge the `dist/` folder and restart Trunk to generate a clean bundle.

