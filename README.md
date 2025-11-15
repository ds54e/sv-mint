# sv-mint

sv-mint is a SystemVerilog lint pipeline that combines a Rust core with Python plugins. It focuses on reproducible diagnostics, predictable resource usage, and easy rule authoring.

## Overview
- **Multi-stage analysis**: raw text, preprocessed text, CST, and AST payloads flow through the pipeline so rules can attach at the right abstraction.
- **Python rule host**: `plugins/lib/rule_host.py` runs once per worker thread and loads every script referenced by `[[rule]]` entries.
- **Deterministic diagnostics**: violations are sorted by file/line, emitted as `path:line:col: [severity] rule_id: message`, and mirrored to `tracing` events for log aggregation.
- **Operational safety**: configurable (default 12/16 MB) size guards, per-file timeouts, stderr snippet limits, and request accounting keep runaway rules in check.


## Getting Started
1. Download the release archive for your platform from the Releases page.
2. Extract the archive and add the `sv-mint` binary to your PATH.
3. Run `sv-mint --config ./sv-mint.toml path/to/files.sv` to lint your sources.
4. Edit `sv-mint.toml` when you need to enable/disable rules or tweak severities.

## Configuration Basics
- `[defaults]` controls `timeout_ms_per_file` and which stages run (raw text, pp_text, cst, ast).
- `[plugin]` selects the Python interpreter and arguments used to launch rule hosts.
- Each `[[rule]]` entry binds a `rule_id` to a plugin script/stage and lets you toggle the rule or override severity.
- CLI helpers: `--only rule_a,rule_b` narrows to select rules, while `--disable rule_x` temporarily removes noisy ones.

## Versioning
- Tags matching `v*` trigger `.github/workflows/release.yml`, which runs fmt/clippy/tests, builds release binaries for Linux/macOS/Windows, and uploads archives containing the binary plus `docs/`, `plugins/`, `sv-mint.toml`, `README.md`, `LICENSE`, and `CHANGELOG.md` (with accompanying SHA-256 checksums).
- Linux artifacts are built with `x86_64-unknown-linux-musl`, so they run on glibc-2.28 era distributions (e.g., RHEL8) without additional dependencies.

## Provenance and License
- This repository and documentation were generated and are maintained with the help of ChatGPT.
- Rust dependencies follow MIT or Apache-2.0 licenses as declared in `Cargo.toml`.
- sv-mint itself is distributed under the same terms as the repository license (see `LICENSE`).
