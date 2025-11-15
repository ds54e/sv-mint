# Developer Guide

This document collects maintainer notes for building sv-mint from source, extending the Rust core, and publishing releases. For user-facing configuration details, see `docs/configuration.md`.

## Building From Source

Install Rust stable (`rustup default stable`) and Python 3.x, then run:

```bash
cargo build --release
target/release/sv-mint --config ./sv-mint.toml path/to/files/*.sv
```

The release workflow bundles `sv-mint`, `docs/`, `plugins/`, `sv-mint.toml`, `README.md`, `LICENSE`, and `CHANGELOG.md` into platform-specific archives.

## Diagnostics and Tooling

- `logging.show_plugin_events = true` exposes per-rule latency for profiling.
- `cargo test --lib` covers Rust logic, while `tests/cli_smoke.rs` runs end-to-end checks using the fixtures under `fixtures/`.
- `cargo clippy --all-targets --all-features` and `cargo fmt --all` must pass before submitting changes.
- Structured logs (`logging.format = "json"`) include `sv-mint::event`, `sv-mint::stage`, and `sv-mint::plugin.stderr` categories for observability pipelines.

## Releasing

1. Update version numbers in `Cargo.toml` and `Cargo.lock`.
2. Document the changes in `CHANGELOG.md`.
3. Run `cargo fmt --all`, `cargo clippy --all-targets --all-features`, and `cargo test --lib`.
4. Commit the release prep, tag it (`git tag vX.Y.Z`), and push both the branch and tag.
5. GitHub Actions builds the archives for Linux/macOS/Windows (Linux uses `x86_64-unknown-linux-musl` for broad compatibility).

## Maintenance Workflow

- Keep README and docs in sync with code changes so plugin authors and users stay aligned.
- When adding new stages or payload fields, update `docs/plugin_author.md` and relevant rule docs under `docs/plugins/`.
- `docs/internal_spec.md` dives deeper into module layout, data contracts, and testing strategy; refer to it when making architectural changes.
