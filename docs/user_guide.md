# sv-mint User Guide

## Intended Audience
This guide targets design and verification engineers or EDA administrators who run sv-mint regularly and need to interpret its reports. It assumes general SystemVerilog and CLI knowledge but does not cover plugin implementation or internal details.

## 1. Setup

### 1.1 Requirements
- Windows 10 or later, macOS, or Linux
- Rust stable toolchain (skip if you only consume published binaries)
- Python 3.x (`python3` or `python` must launch it)
- UTF-8 editor (BOM accepted)

### 1.2 Build Steps
```
rustup default stable
cargo build --release
```
After building, place `target/release/sv-mint` (or `.exe` on Windows) somewhere on your PATH. For CI speedups, prefer `cargo build --release --locked`.

### 1.3 Python Runtime
The Python host is launched according to the `[plugin]` `cmd` / `args`. By default sv-mint spawns `python3 -u -B plugins/lib/rule_host.py`, so point `cmd` to the virtualenv interpreter when needed.

## 2. Running the CLI

### 2.1 Basic Invocation
```
sv-mint --config ./sv-mint.toml path/to/a.sv path/to/b.sv
```
- Omitting `--config` loads `sv-mint.toml` from the current directory.
- Multiple inputs are supported; two or more files run in parallel worker threads.

### 2.2 Exit Codes
| Code | Description |
| --- | --- |
| 0 | No violations |
| 2 | Violations detected (warning or error) |
| 3 | Fatal issue such as config parse failure, file parse failure, plugin crash, or timeout |

### 2.3 Common Options
| Option | Description |
| --- | --- |
| `--config <path>` | Explicitly select the TOML configuration file |
| `<input ...>` | SystemVerilog files or globs to lint |

## 3. `sv-mint.toml` Configuration
Write all values in TOML format. The main sections are:

### 3.1 `[defaults]`
| Key | Type | Description |
| --- | --- | --- |
| `timeout_ms_per_file` | u64 | Plugin timeout per file (milliseconds) |

### 3.2 `[plugin]`
| Key | Type | Description |
| --- | --- | --- |
| `cmd` | string | Python executable (e.g., `python3`) |
| `args` | [string] | Extra arguments when launching `rule_host.py` |

### 3.3 `[ruleset]`
| Key | Description |
| --- | --- |
| `scripts` | Array of Python rule paths; load order equals execution order |
| `override` | Map from rule ID to `error|warning|info`, overriding CLI severities |

### 3.4 `[stages]`
| Key | Description |
| --- | --- |
| `enabled` | Select which stages to run: `raw_text`, `pp_text`, `cst`, `ast` |

### 3.5 `[svparser]`
| Key | Description |
| --- | --- |
| `include_paths` | Equivalent to `+incdir` entries passed to `sv-parser` |
| `defines` | Predefined `` `define`` entries in `NAME=VALUE` form |
| `strip_comments` | Remove comments before preprocessing |
| `ignore_include` | Treat inputs as standalone files, ignoring `include` |
| `allow_incomplete` | Allow incomplete constructs inside the parser |

### 3.6 `[logging]`
| Key | Description |
| --- | --- |
| `level` | `error`, `warn`, `info`, `debug`, or `trace` |
| `format` | `text` or `json` |
| `stderr_snippet_bytes` | Bytes of plugin stderr to retain (0 disables) |
| `show_*_events` | Toggle `stage`, `plugin`, and `parse` event logs |

### 3.7 Templates
Use the [sample config](../README.md#3-quick-start) from the README as a base and swap in organization-specific include paths and rule sets.

## 4. Reading Logs and Diagnostics

### 4.1 CLI Output
Each violation is printed as `<path>:<line>:<col>: [severity] <rule_id>: <message>`. When `location.file` is present, includes show their original path.

### 4.2 `tracing` Events
- `sv-mint::event`: Stage start/end, plugin invoke/done, timeout, stderr snippet, etc. via info logs.
- `sv-mint::stage`: Per-stage stats such as elapsed time, violation count, skip/fail flags.
Set `logging.format = "json"` to collect structured events.

### 4.3 Size Guards and Timeouts
- Request JSON larger than 12 MB emits a warning; exceeding 16 MB fails required stages and skips optional ones.
- Plugin timeouts terminate the process, emit a `PluginTimeout` event, and produce a `sys.stage.timeout` diagnostic.

## 5. FAQ (Excerpt)

**Q. Can I analyze files that still have CRLF on Windows?**  
A. Yes. `read_input` normalizes to LF while all byte offsets reference the original file.

**Q. How do I tweak severity per rule?**  
A. Add entries like `"rule.id" = "warning"` under `[ruleset.override]` to rewrite CLI severities without touching the plugin code.

**Q. How do I see filenames for violations inside included files?**  
A. Since v2.7 the actual path is recorded in `Location.file` and printed by the CLI. Older versions required access to the raw source bytes.

For payload specs, protocol details, and architecture notes, see `docs/plugin_author.md` or `docs/internal_spec.md`.
