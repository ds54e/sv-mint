# sv-mint

sv-mint is a SystemVerilog lint pipeline that combines a Rust core with Python plugins. It focuses on reproducible diagnostics, predictable resource usage, and easy rule authoring.

## Overview
- **Multi-stage analysis**: raw text, preprocessed text, CST, and AST payloads flow through the pipeline so rules can attach at the right abstraction.
- **Python rule host**: `plugins/lib/rule_host.py` runs once per worker thread and loads every script referenced by `[[rule]]` entries.
- **Deterministic diagnostics**: violations are sorted by file/line, emitted as `path:line:col: [severity] rule_id: message`, and mirrored to `tracing` events for log aggregation.
- **Operational safety**: configurable (default 12/16 MB) size guards, per-file timeouts, stderr snippet limits, and request accounting keep runaway rules in check.

## Getting Started
1. Download the latest release from GitHub (`sv-mint-vX.Y.Z-<platform>.tar.gz`/`.zip`) and extract it somewhere on your machine.
2. Add the extracted directory (it contains `sv-mint`, `docs/`, `plugins/`, `sv-mint.toml`, and `LICENSE`) to your `PATH`, or call the binary via an absolute path.
3. Lint your sources:
   ```bash
   ./sv-mint --config ./sv-mint.toml path/to/files/*.sv
   ```
4. Tailor rules by editing `sv-mint.toml`. A section-by-section reference (including default values and stage behavior) lives in [`docs/configuration.md`](docs/configuration.md). The short version: declare your `[[rule]]` entries, point `[plugin]` at your Python interpreter, and let sv-mint’s built-in defaults cover everything else unless you need overrides.

### Sample `sv-mint.toml`

```toml
[[rule]]
id = "macro_names_uppercase"

[[rule]]
id = "vars_not_left_unused"
```

See [`docs/configuration.md`](docs/configuration.md) for every optional section, stage inference, and script resolution rules.

5. Narrow or relax checks directly from the CLI when experimenting:
   - `sv-mint --only rule_x path/to/file.sv` runs only `rule_x`, temporarily disabling every other rule.
   - `sv-mint --disable rule_a,rule_b path/to/file.sv` disables just the listed rules; specify multiple IDs or repeat `--disable` as needed.
   - When `--only` is present, any `--disable` that follows removes rules from that already-filtered set, and referencing a nonexistent `rule_id` raises an error.

## Anatomy of a Rule
- Rules live under `plugins/` and expose a `check(req)` function.
- `req.stage` decides which payload type (`raw_text`, `pp_text`, `cst`, `ast`) is available.
- Return a list of `Violation` dictionaries with `rule_id`, `severity`, `message`, and `location`.
- Document every bundled rule in `docs/plugins/<rule_id>.md` so users know how to remediate findings.
- For project-specific rules, add subdirectories inside `plugins/` and point `sv-mint.toml` at the new scripts.

## Diagnostics and Tooling
- Use `logging.show_plugin_events = true` to measure per-rule latency.
- Structured logs (`logging.format = "json"`) expose `sv-mint::event`, `sv-mint::stage`, and `sv-mint::plugin.stderr` categories for observability platforms.

## Provenance and License
- This repository and documentation were generated and are maintained with the help of ChatGPT.
- Rust dependencies follow MIT or Apache-2.0 licenses as declared in `Cargo.toml`.
- sv-mint itself is distributed under the same terms as the repository license (see `LICENSE`).
- Internal maintenance notes (builds, releases, tests) are tracked in `docs/development.md`.
