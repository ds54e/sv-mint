# sv-mint

sv-mint is a SystemVerilog lint pipeline that combines a Rust core with Python plugins. It focuses on reproducible diagnostics, predictable resource usage, and easy rule authoring.

## Overview
- **Multi-stage analysis**: raw text, preprocessed text, CST, and AST payloads flow through the pipeline so rules can attach at the right abstraction.
- **Python rule host**: `plugins/lib/rule_host.py` runs once per worker thread and loads every script referenced by `[[rule]]` entries.
- **Deterministic diagnostics**: violations are sorted by file/line, emitted as `path:line:col: [severity] rule_id: message`, and mirrored to `tracing` events for log aggregation.
- **Operational safety**: configurable (default 12/16 MB) size guards, per-file timeouts, stderr snippet limits, and request accounting keep runaway rules in check.

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
   - `[transport]` defines request/response byte limits, warning margins, and how strictly to treat size overruns; mark critical stages under `[stages.required]` to fail fast.
5. Narrow or relax checks directly from the CLI when experimenting:
   - `sv-mint --only rule_x path/to/file.sv` runs only `rule_x`, temporarily disabling every other rule.
   - `sv-mint --disable rule_a,rule_b path/to/file.sv` disables just the listed rules; specify multiple IDs or repeat `--disable` as needed.
   - When `--only` is present, any `--disable` that follows removes rules from that already-filtered set, and referencing a nonexistent `rule_id` raises an error.
6. Feed filelists via `-f/--filelist`. Entries can nest other lists with `-f`, inject include paths with `+incdir+`, and add defines with `+define+`. Relative paths are resolved against the filelist location before being passed to `sv-parser`.

### Filelists

`sv-mint -f path/to/files.f` consumes filelist entries using a small, svlint-like syntax:
- `-f child.f` (or `-fchild.f`) recursively loads another list; cycles raise `invalid value` errors.
- `+incdir+dir1+dir2` appends include directories to `svparser.include_paths` after resolving each path relative to the filelist; environment variables like `${IP_ROOT}` or `$(OUTDIR)` are expanded before resolution.
- `+define+FOO=1+BAR` appends raw `name` or `name=value` strings to `svparser.defines`, matching the inline CLI format; multi-line entries can be continued with a trailing `\`.
- Any other non-empty, non-comment line is treated as an input file path and queued for linting.
- Comments beginning with `//` or `#`, plus blank lines, are ignored.
- Windows drive-letter (`C:\proj\foo.sv`) and UNC (`\\server\share\bar.sv`) paths are treated as absolute even on Unix hosts, so mixed-platform filelists work out of the box.

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

## Versioning
- Current release: **1.0.0**. Use semantic versioning going forward so downstream automation can pin to stable rule/transport behavior.
- Tags matching `v*` trigger `.github/workflows/release.yml`, which runs fmt/clippy/tests, builds release binaries for Linux/macOS/Windows, and uploads archives containing the binary plus `docs/`, `plugins/`, `sv-mint.toml`, `README.md`, `LICENSE`, and `CHANGELOG.md` (with accompanying SHA-256 checksums).
- Linux artifacts are built with `x86_64-unknown-linux-musl`, so they run on glibc-2.28 era distributions (e.g., RHEL8) without additional dependencies.

## Provenance and License
- This repository and documentation were generated and are maintained with the help of ChatGPT.
- Rust dependencies follow MIT or Apache-2.0 licenses as declared in `Cargo.toml`.
- sv-mint itself is distributed under the same terms as the repository license (see `LICENSE`).
