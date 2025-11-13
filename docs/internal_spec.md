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

## 2. Pipeline Details
```
Pipeline::run_files -> run_file_batch/run_files_parallel -> run_file_with_host
```
1. `io::config::read_input` loads UTF-8 text, strips BOM, and normalizes line endings while returning both the original and normalized text.
2. `sv::driver::SvDriver` invokes `sv-parser` to create `ParseArtifacts`, bundling raw_text / pp_text / CST / AST / defines. `sv::source::SourceCache` tracks line maps for includes so violations get the correct `Location.file`.
3. `core::payload::StagePayload` builds JSON payloads per stage. Before serialization, `core::size_guard::enforce_request_size` checks the 12/16 MB thresholds.
4. `plugin::client::PythonHost` starts `python3` within a `tokio` runtime and speaks NDJSON with `rule_host.py`. On timeout it uses `start_kill`, emitting a `PluginTimeout` event.
5. Plugin responses merge into a `Vec<Violation>` for CLI display. stderr beyond `logging.stderr_snippet_bytes` keeps only the tail in logs.

## 3. Data Contracts
### 3.1 `StagePayload`
- `raw_text`: direct copy of `InputText.normalized`.
- `pp_text`: preprocessed text plus the list of `DefineInfo`.
- `cst`: `cst_ir` (`CstIr`) or a `has_cst` flag. Send `mode: "none"` when `sv-parser` does not return a CST.
- `ast`: `AstSummary` with `decls` / `refs` / `symbols` / `assigns` / `ports` / `pp_text`, etc. Note that `line_map` is intentionally omitted from serialization.

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

### 3.3 Error Types
| Type | Purpose |
| --- | --- |
| `ConfigError` | Config parsing, validation, or IO failures |
| `ParseError` | Failures during preprocessing, parsing, or CST extraction |
| `PluginError` | Python host startup, IO, JSON decoding, or timeout issues |
| `OutputError` | CLI output helpers (mainly for `io::output` tests) |

## 4. Logging and Events
- `diag::logging::init` builds a `tracing_subscriber` according to `LoggingConfig.format` / `level`. Unknown keys under `extra` trigger warnings.
- Events are modeled by the `Event` enum and funneled through `Ev::log_event`. `show_*_events` lets users suppress categories.
- `PluginStderr` snippets are disabled when `logging.stderr_snippet_bytes` is 0.

## 5. Parallel Execution
- `Pipeline::run_files_parallel` uses `std::thread::scope` and `available_parallelism` to spawn workers. Each worker owns its own `PythonHost`, so plugins must be reentrant.
- When the number of files is smaller than logical CPUs, the thread count shrinks accordingly.

## 6. Size Guards and Response Limits
- Constants: `MAX_REQ_BYTES = 16_000_000`, `WARN_REQ_BYTES = 12_000_000`. Required stages (`raw_text`, `pp_text`) over the limit emit `Severity::Error` with `sys.stage.skipped.size` and abort processing.
- Responses also check the 16 MB limit via `enforce_response_size`. Violations raise `sys.stage.output.too_large`.

## 7. Error Propagation and Diagnostics
- When `sv::driver::parse_text` fails, it reports `sys.parse.failed`, streams the diagnostic immediately, sets `summary.had_error`, and returns exit code 3.
- Plugin exceptions surface as `PluginError::ProtocolError` while logs capture a `PluginExitNonzero` event.

## 8. Extension Guidelines
- Adding a stage requires updates to `types::Stage`, `StagePayload`, and the TOML `stages.enabled` parsing.
- When exposing new payload fields to rule authors, update `docs/plugin_author.md` and the relevant `docs/plugins/<script>.md` entry in lockstep.
- If size guard thresholds become configurable, modify both the TOML validation logic and the `SizePolicy` constructor.

## 9. Testing Strategy
- `tests/cli_smoke.rs` performs E2E coverage for major rules. Add fixtures and entries here when introducing new rules.
- Rust unit tests are sparse; add `#[cfg(test)]` blocks as needed when modifying subsystems.
- Test Python rules via standalone `pytest` runs or by wiring fixtures into the CLI tests.

## 10. Future Ideas
- Configurable size-guard thresholds
- JSON summaries of pipeline results
- Alternative transports (e.g., replacing the Python host with gRPC)

Keep README and the docs directory updated whenever you make core changes so users and contributors stay aligned.
