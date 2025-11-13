# sv-mint

sv-mint is a SystemVerilog lint pipeline that combines a Rust core with Python plugins. It focuses on reproducible diagnostics, predictable resource usage, and easy rule authoring.

## Overview
- **Multi-stage analysis**: raw text, preprocessed text, CST, and AST payloads flow through the pipeline so rules can attach at the right abstraction.
- **Python rule host**: `plugins/lib/rule_host.py` runs once per worker thread and loads every script listed under `[ruleset.scripts]`.
- **Deterministic diagnostics**: violations are sorted by file/line, emitted as `path:line:col: [severity] rule_id: message`, and mirrored to `tracing` events for log aggregation.
- **Operational safety**: 12/16 MB size guards, per-file timeouts, stderr snippet limits, and request accounting keep runaway rules in check.

## Getting Started
1. Install Rust stable, Python 3.x, and a recent sv-parser-compatible toolchain.
2. Build the project:
   ```bash
   rustup default stable
   cargo build --release
   ```
3. Run the CLI against your sources:
   ```bash
   target/release/sv-mint --config ./sv-mint.toml path/to/files/*.sv
   ```
4. Tailor rules by editing `sv-mint.toml`:
   - `[defaults]` sets `timeout_ms_per_file` and stage toggles.
   - `[plugin]` selects the Python interpreter/arguments.
   - `[ruleset]` lists scripts, severity overrides, and allowlists.
   - `[logging]` controls `level`, `format` (`text|json`), and event visibility.

## Anatomy of a Rule
- Rules live under `plugins/` and expose a `check(req)` function.
- `req.stage` decides which payload type (`raw_text`, `pp_text`, `cst`, `ast`) is available.
- Return a list of `Violation` dictionaries with `rule_id`, `severity`, `message`, and `location`.
- Document every bundled rule in `docs/plugins/<script>.md` so users know how to remediate findings.
- For project-specific rules, add subdirectories inside `plugins/` and point `sv-mint.toml` at the new scripts.

## Diagnostics and Tooling
- Use `logging.show_plugin_events = true` to measure per-rule latency.
- Integration tests live in `tests/cli_smoke.rs` and rely on fixtures under `fixtures/`.
- Structured logs (`logging.format = "json"`) expose `sv-mint::event`, `sv-mint::stage`, and `sv-mint::plugin.stderr` categories for observability platforms.

## Future Ideas
1. **Configurable size guards**: expose request/response thresholds via `sv-mint.toml`.
2. **JSON run reports**: emit machine-readable summaries for CI dashboards.
3. **Alternative transports**: explore gRPC or IPC sockets instead of spawning Python hosts per worker.
4. **Deeper semantic rules**: add bit-width analysis, dependency graphs, and state-machine coverage checks.

## Provenance and License
- This repository and documentation were generated and are maintained with the help of ChatGPT.
- Rust dependencies follow MIT or Apache-2.0 licenses as declared in `Cargo.toml`.
- sv-mint itself is distributed under the same terms as the repository license (see `LICENSE`).
