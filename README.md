# sv-mint

sv-mint is a Rust-based SystemVerilog lint pipeline. It streams the raw_text / pp_text / CST / AST payloads produced by `sv-parser` into Python plugins, while providing operational features such as parallel execution, size guards, and per-stage timeouts.

## Table of Contents
- [1. Overview](#1-overview)
- [2. Role-Based Quick Links](#2-role-based-quick-links)
- [3. Quick Start](#3-quick-start)
- [4. Processing Flow](#4-processing-flow)
- [5. Development Resources](#5-development-resources)
- [6. Logging and Size Guards](#6-logging-and-size-guards)
- [7. Output and Operations Notes](#7-output-and-operations-notes)
- [8. Provenance and License](#8-provenance-and-license)

## 1. Overview
sv-mint lets design and verification teams enforce a unified SystemVerilog rule set.

Highlights:
- Multi-stage pipeline that dispatches raw_text / pp_text / CST / AST payloads to plugins
- Resident Python host parallelizes the `ruleset.scripts` rules
- 16 MB request/response size guards and per-stage timeouts
- Structured and plain-text logs via `tracing`, plus stderr snippet capture
- Identical behavior on Windows / macOS / Linux (UTF-8, CRLF/LF tolerant)

## 2. Role-Based Quick Links

| Reader | Focus | Document |
| --- | --- | --- |
| First-time users | CLI usage, `sv-mint.toml`, FAQ | [docs/user_guide.md](docs/user_guide.md) |
| Anyone researching rule specs | Stage, severity, remediation, and detection logic per `rule_id` | [docs/plugins/](docs/plugins) |
| Plugin developers | Payload schema, violation format, debugging tips | [docs/plugin_author.md](docs/plugin_author.md) |
| Rust core contributors | Core structure, data contracts, error taxonomy | [docs/internal_spec.md](docs/internal_spec.md) |

## 3. Quick Start

### 3.1 Requirements
- OS: Windows 10+, Linux, or macOS
- Rust: stable toolchain
- Python: 3.x (`python3` must be available)
- Encoding: UTF-8 (BOM allowed), either LF or CRLF endings

### 3.2 Build
```bash
rustup default stable
cargo build --release
```
The binary is written to `target/release/sv-mint` (or `.exe` on Windows).

### 3.3 Run
```bash
sv-mint --config ./sv-mint.toml path/to/file.sv
```
If `--config` is omitted, the CLI loads `sv-mint.toml` from the working directory.

### 3.4 Exit Codes
| Code | Meaning |
| ---- | ------- |
| 0 | No diagnostics |
| 2 | Violations detected (warnings or errors) |
| 3 | Fatal issues such as invalid input, config, plugin crash, or timeout |

## 4. Processing Flow
1. Normalize the input to UTF-8 and run `sv-parser` preprocessing (`pp_text`) and parsing.
2. Produce `StagePayload` objects for raw_text / pp_text / CST / AST and enforce JSON size limits.
3. Send NDJSON requests to the resident host `plugins/lib/rule_host.py`, executing each `ruleset.scripts` entry via `check(req)`.
4. Aggregate returned violations and emit them to the CLI output and `tracing` events simultaneously.

See [docs/plugin_author.md](docs/plugin_author.md) and [docs/internal_spec.md](docs/internal_spec.md) for payload and host details.

## 5. Development Resources
- `docs/user_guide.md#3-sv-minttoml-configuration`: template and option breakdown for `sv-mint.toml`.
- Files like `docs/plugins/lang_construct_rules.md`: deep dives into each `rule_id` reported by the CLI.
- `fixtures/`: SystemVerilog samples for regression tests or rule experimentation.
- `tests/cli_smoke.rs`: end-to-end coverage for representative rules.
- `plugins/`: bundled rules, `rule_host.py` hot-reload helpers, and utilities such as `debug_ping.py`.

Feedback is welcome via Issues or Pull Requests.

## 6. Logging and Size Guards

### 6.1 Logging
- `logging.level` feeds into the `tracing` filter, emitting `sv-mint::event` (pipeline events), `sv-mint::stage` (stage results), and `sv-mint::logging` (config warnings).
- `logging.format = "json"` enables structured logs; unsupported options trigger warning logs.
- Flags like `show_stage_events` and `show_plugin_stderr` control debug verbosity.

### 6.2 Supported Rule Examples
- Text formatting such as line length, ASCII restriction, tab bans, and preprocessing hygiene
- Naming: module/signal/port conventions (`clk/rst` ordering, diff pairs, pipeline `_q2`), `typedef` `_e/_t`, UpperCamelCase parameters
- Behavioral guidance: prefer `always_comb`, enforce async resets in `always_ff`, forbid `always_latch` / `always @*`, require `case` `default`/`unique`, detect multiple non-blocking assignments
- Governance checks: `` `define`` inside `package`, multiple `package` declarations, module instance `.*` / positional ports, SPDX headers, global `` `define`` policies

### 6.3 Possible Future Work
- Bit-width analysis (preventing boolean use of multi-bit signals, inferring widths)
- Coverage for `unique/priority case`, handling `casez/casex`, tracing `X` propagation
- Detecting `package` dependency cycles and advanced `parameter/localparam` validation
- Stricter formatting such as aligned comments and detailed style rules

## 7. Output and Operations Notes

### 7.1 Preventing Bytecode
- Pass `-B` to Python hosts (see default args).
- Export `PYTHONDONTWRITEBYTECODE=1`.
- `.gitignore` already lists `__pycache__/` and `*.pyc`.

### 7.2 Diagnostic Format
```
<path>:<line>:<col>: [<severity>] <rule_id>: <message>
```
If any violation is reported, the CLI exits with status 2. To lint multiple files, pass a glob such as `cargo run -- fixtures/*.sv`.

## 8. Provenance and License
- This software and its documentation were produced with ChatGPT.
- Dependencies are MIT or Apache-2.0 licensed; see Cargo.toml for details.
