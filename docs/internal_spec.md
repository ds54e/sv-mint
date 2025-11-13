# Internal Specification

## 0. Audience
This document targets sv-mint developers, design leads, or engineers planning to extend the core. It describes the Rust module layout, data contracts, error taxonomy, and extension caveats.

## 1. Module Layout
| Directory | Role |
| --- | --- |
| `src/lib.rs` | Entry point that wires public modules such as `core/`, `sv/`, `io/`, and `plugin/`. |
| `src/bin/sv-mint.rs` | CLI (`clap`) definition and `Pipeline` launcher that maps exit codes. |
| `src/core/` | Pipeline orchestration, payload builders, size guards, diagnostic types. |
| `src/io/` | Config loading, text helpers, CLI output formatting. |
| `src/sv/` | `sv-parser` bridge, CST/AST builders, line-map management. |
| `src/plugin/` | IPC client that talks to the Python host via a tokio runtime. |
| `plugins/` | Rule implementations plus `rule_host.py`. |
| `fixtures/` / `tests/` | CLI integration tests and SystemVerilog samples. |

### 1.1 Core Submodules
| Module | Highlights |
| --- | --- |
| `core::pipeline` | Coordinates per-file stages, manages worker threads, aggregates results. |
| `core::payload` | Converts parser artifacts into JSON-friendly structures and enforces size guards. |
| `core::diagnostic` | Defines `Violation`, `StageSummary`, and sys.* helper IDs. |
| `io::config` | Parses `sv-mint.toml` via `toml_edit`, validates sections, resolves relative paths. |
| `plugin::client` | Wraps the rule host process, handles NDJSON framing, timeout/kill logic. |
| `diag::logging` | Bootstraps `tracing_subscriber` and exposes CLI flags for filtering events. |

## 2. Pipeline Details
```
Pipeline::run_files -> run_file_batch/run_files_parallel -> run_file_with_host
```
1. `io::config::read_input` loads UTF-8 text, strips BOM, and normalizes line endings while returning both the original and normalized text.
2. `sv::driver::SvDriver` invokes `sv-parser` to create `ParseArtifacts`, bundling raw_text / pp_text / CST / AST / defines. `sv::source::SourceCache` tracks line maps for includes so violations get the correct `Location.file`.
3. `core::payload::StagePayload` builds JSON payloads per stage. Before serialization, `core::size_guard::enforce_request_size` checks the 12/16 MB thresholds.
4. `plugin::client::PythonHost` starts `python3` within a `tokio` runtime and speaks NDJSON with `rule_host.py`. On timeout it uses `start_kill`, emitting a `PluginTimeout` event.
5. Plugin responses merge into a `Vec<Violation>` for CLI display. stderr beyond `logging.stderr_snippet_bytes` keeps only the tail in logs.

### 2.1 Execution Timeline
```
CLI args
  ↓
Config load & validation
  ↓
Input batching (per stage list)
  ↓
Worker thread picks file
  ↓
Stage loop:
    build payload
    enforce size guards
    send NDJSON to rule host
    receive violation array
  ↓
Aggregate violations + stage stats
  ↓
Emit diagnostics (stdout + tracing)
  ↓
Summaries determine exit code
```

Stage order is derived from `[stages.enabled]`. A rule host is lazily spawned per worker the first time a plugin stage runs.

## 3. Data Contracts
### 3.1 `StagePayload`
- `raw_text`: direct copy of `InputText.normalized`.
- `pp_text`: preprocessed text plus the list of `DefineInfo`.
- `cst`: `cst_ir` (`CstIr`) or a `has_cst` flag. Send `mode: "none"` when `sv-parser` does not return a CST.
- `ast`: `AstSummary` with `decls` / `refs` / `symbols` / `assigns` / `ports` / `pp_text`, etc. Note that `line_map` is intentionally omitted from serialization.

Additional metadata:
- `file_id`: stable identifier for cross-referencing includes.
- `defines[*].origin`: indicates whether the define came from CLI `--define` or in-source macros.
- `assigns[*].op`: `blocking`, `nonblocking`, or `continuous`.

### 3.2 `Violation`
```rust
pub struct Location {
    pub line: u32,
    pub col: u32,
    pub end_line: u32,
    pub end_col: u32,
    pub file: Option<String>,
}
```
CLI output uses `location.file.unwrap_or(input_path)`. `SourceCache` must populate `file` for include support.

Severity semantics:
- `error`: contributes to exit code 2 and is treated as a blocking finding.
- `warning`: still elevates exit code to 2 but signals lower urgency.
- `info`: informational; exit code stays 0 unless other findings exist.

### 3.3 Error Types
| Type | Purpose |
| --- | --- |
| `ConfigError` | Config parsing, validation, or IO failures |
| `ParseError` | Failures during preprocessing, parsing, or CST extraction |
| `PluginError` | Python host startup, IO, JSON decoding, or timeout issues |
| `OutputError` | CLI output helpers (mainly for `io::output` tests) |

## 4. Configuration Ingestion
- The CLI accepts `--config <path>` (default `./sv-mint.toml`). Values cascade across:
  - `[defaults]`: timeouts, response limits, stage selection.
  - `[plugin]`: interpreter, args, environment overrides.
  - `[[rule]]`: per-rule metadata (id, script path, stage, `enabled`, `severity`, future allowlists).
  - `[logging]`: `level`, `format`, `show_*_events`, `stderr_snippet_bytes`.
- Relative paths inside the config are resolved against the config directory.
- Invalid keys surface via `LoggingConfig.extra` warnings so we fail fast when typos occur.

## 4. Logging and Events
- `diag::logging::init` builds a `tracing_subscriber` according to `LoggingConfig.format` / `level`. Unknown keys under `extra` trigger warnings.
- Events are modeled by the `Event` enum and funneled through `Ev::log_event`. `show_*_events` lets users suppress categories.
- `PluginStderr` snippets are disabled when `logging.stderr_snippet_bytes` is 0.

### 4.1 Event Taxonomy
| Event | When Emitted | Fields |
| --- | --- | --- |
| `sv-mint::event` | Stage start/end, plugin invoke/done, size guard warnings | `stage`, `path`, `duration_ms`, `plugin` |
| `sv-mint::stage` | After each stage completes | `violations`, `skipped`, `timeout`, `req_bytes` |
| `sv-mint::logging` | Config or runtime warnings | `code` (e.g., `config.unknown_key`), `detail` |
| `sv-mint::plugin.stderr` | When stderr snippets are flushed | `rule_id`, `bytes` |

Set `logging.format = "json"` to integrate with structured log collectors (Splunk, Stackdriver, etc.).

## 5. Parallel Execution
- `Pipeline::run_files_parallel` uses `std::thread::scope` and `available_parallelism` to spawn workers. Each worker owns its own `PythonHost`, so plugins must be reentrant.
- When the number of files is smaller than logical CPUs, the thread count shrinks accordingly.
- Host processes run with `PYTHONDONTWRITEBYTECODE=1` and `-B` to avoid polluting the repo.
- Stage order is preserved even when worker threads overlap: `run_file_with_host` blocks until the current file completes all enabled stages.

## 6. Size Guards and Response Limits
- Constants: `MAX_REQ_BYTES = 16_000_000`, `WARN_REQ_BYTES = 12_000_000`. Required stages (`raw_text`, `pp_text`) over the limit emit `Severity::Error` with `sys.stage.skipped.size` and abort processing.
- Responses also check the 16 MB limit via `enforce_response_size`. Violations raise `sys.stage.output.too_large`.
- Size guard decisions are recorded in `StageSummary::skipped_reason` so downstream tooling can highlight truncated stages.

## 7. Error Propagation and Diagnostics
- When `sv::driver::parse_text` fails, it reports `sys.parse.failed`, streams the diagnostic immediately, sets `summary.had_error`, and returns exit code 3.
- Plugin exceptions surface as `PluginError::ProtocolError` while logs capture a `PluginExitNonzero` event.
- Exit codes:
  - `0`: no violations, no fatal errors.
  - `2`: at least one rule violation (warning or error).
  - `3`: configuration errors, parser failures, plugin crashes, or timeouts.
- Fatal errors short-circuit remaining stages for the affected file but the pipeline still attempts other queued files so users see as many findings as possible.

## 8. Extension Guidelines
- Adding a stage requires updates to `types::Stage`, `StagePayload`, and the TOML `stages.enabled` parsing.
- When exposing new payload fields to rule authors, update `docs/plugin_author.md` and the relevant `docs/plugins/<script>.md` entry in lockstep.
- If size guard thresholds become configurable, modify both the TOML validation logic and the `SizePolicy` constructor.
- When touching the parser integration, add fixtures under `fixtures/` and cross-link them in docs so plugin authors understand the new shape of payloads.

## 9. Testing Strategy
- `tests/cli_smoke.rs` performs E2E coverage for major rules. Add fixtures and entries here when introducing new rules.
- Rust unit tests are sparse; add `#[cfg(test)]` blocks as needed when modifying subsystems.
- Test Python rules via standalone `pytest` runs or by wiring fixtures into the CLI tests.
- Use `cargo test --lib core::payload` to verify serialization when changing StagePayload structures.
- `scripts/capture_payload.sh` (dev helper) dumps raw JSON for regression purposes; keep large dumps out of git but reference them in PR notes.

## 10. Future Ideas
- Configurable size-guard thresholds
- JSON summaries of pipeline results
- Alternative transports (e.g., replacing the Python host with gRPC)

Keep README and the docs directory updated whenever you make core changes so users and contributors stay aligned.
