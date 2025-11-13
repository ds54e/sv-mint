# sv-mint

sv-mint is a SystemVerilog lint pipeline that combines a Rust core with Python plugins. It focuses on reproducible diagnostics, predictable resource usage, and easy rule authoring.

## Overview
- **Multi-stage analysis**: raw text, preprocessed text, CST, and AST payloads flow through the pipeline so rules can attach at the right abstraction.
- **Python rule host**: `plugins/lib/rule_host.py` runs once per worker thread and loads every script referenced by `[[rule]]` entries.
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
   - `[[rule]]` entries bind each `rule_id` to a script, stage, `enabled` flag, and optional severity override.
   - `[logging]` controls `level`, `format` (`text|json`), and event visibility.
5. Narrow or relax checks directly from the CLI when experimenting:
   - `sv-mint --only rule_x path/to/file.sv` は `rule_x` のみ実行し、他のルールを一時的に無効化します。
   - `sv-mint --disable rule_a,rule_b path/to/file.sv` は列挙したルールだけを無効化します（複数指定や複数回の `--disable` も可）。
   - `--only` 適用後に `--disable` を併用すると「実行対象にしたルール集合からさらに一部を OFF」という順序で評価され、存在しない `rule_id` を指定するとエラーになります。

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

## Comparison with svlint and Verible
- **sv-mint**: Primary focus—multi-stage pipeline with deterministic diagnostics; Rule authoring—Python plugins loaded via `sv-mint.toml` with access to raw/CST/AST payloads; Extensibility—add custom scripts without recompiling and mix stages per rule; Notable strengths—tight size/time guards, reproducible ordering, Rust host guarantees.
- **svlint**: Primary focus—lightweight textual linting (https://github.com/dalance/svlint); Rule authoring—TOML-configured regex/pattern checks compiled into the Rust binary; Extensibility—extend by contributing Rust code or running external commands; Notable strengths—simple setup that enforces static style rules.
- **Verible**: Primary focus—comprehensive formatting/lint suite (https://chipsalliance.github.io/verible); Rule authoring—C++ rules integrated with the parser and configured via flags or waiver files; Extensibility—extend by modifying C++ passes (third-party plugins are uncommon); Notable strengths—mature parser, auto-formatter, and Bazel/IDE integrations.

sv-mint complements these tools: use svlint for quick static checks, Verible for formatting and structural lint, and sv-mint when you need reproducible, Python-authored policies tied to specific pipeline stages.

## Future Ideas
1. **Configurable size guards**: expose request/response thresholds via `sv-mint.toml`.
2. **JSON run reports**: emit machine-readable summaries for CI dashboards.
3. **Alternative transports**: explore gRPC or IPC sockets instead of spawning Python hosts per worker.
4. **Deeper semantic rules**: add bit-width analysis, dependency graphs, and state-machine coverage checks.

## Provenance and License
- This repository and documentation were generated and are maintained with the help of ChatGPT.
- Rust dependencies follow MIT or Apache-2.0 licenses as declared in `Cargo.toml`.
- sv-mint itself is distributed under the same terms as the repository license (see `LICENSE`).
