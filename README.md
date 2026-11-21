# WinIsland

Native Dynamic Island for Windows 11. Rust + Windows.UI.Composition + Direct2D.

## Build (Windows 11, Rust stable, MSVC build tools)

    cargo run --release

If blur doesn't show, set `force_solid_backdrop = true` in `%APPDATA%\WinIsland\settings.toml`.

Controls (dev): left-click toggles pill / expanded, Esc collapses then quits.

## Tests

    cargo test        # reducer + hit-test + layout; core logic needs no Windows APIs

## Status

Phases 1-2 scaffolded: window, composition tree, spring morphing, state machine.
Everything marked `TODO(phase N)` is a stub for that roadmap phase.
See `docs/architecture.md`.
