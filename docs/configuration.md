# Configuring `sv-mint.toml`

`sv-mint` reads its configuration from `sv-mint.toml` (override with `sv-mint --config path/to/file`). Every section except `[[rule]]` is optional: any omitted tables fall back to the defaults listed below, so you can start from a minimal config that only declares rules.

## `[defaults]`

| key | description | default |
| --- | --- | --- |
| `timeout_ms_per_file` | Budget for every file across all stages. Rules that exceed the budget abort the run. | `6000` |

Values below 100 or above 60000 are rejected during config validation.
The timeout is applied per stage (each stage gets the same budget), not as a single cumulative budget across stages.

## `[plugin]`

Controls the Python worker that hosts rule scripts.

| key | description | default |
| --- | --- | --- |
| `cmd` | Interpreter path. | `python3` |
| `args` | Extra arguments passed before the host script. | `["-u", "-B"]` |
| `root` | Base directory used to resolve relative `script` paths. | _unset_ |
| `search_paths` | Additional directories searched for every rule script. | `[]` |

`cmd` must be non-empty; otherwise config validation fails.

When `root` and `search_paths` are not provided, sv-mint looks under `./plugins` relative to the directory that holds `sv-mint.toml`. If a rule omits `script`, the loader searches these directories for `<rule_id>.<stage>.py`. When `root` is set, `plugins/lib/rule_host.py` and `<rule_id>.<stage>.py` are resolved under that root; specify `search_paths` only when you want to add extra lookup locations.

## `[logging]`

| key | description | default |
| --- | --- | --- |
| `level` | Tracing level forwarded to `tracing_subscriber`. | `info` |
| `stderr_snippet_bytes` | Maximum bytes from plugin stderr kept for diagnostics. | `2048` |
| `show_stage_events` | Emit `stage_*` events. | `false` |
| `show_plugin_events` | Emit plugin invoke/done/timeout messages. | `false` |
| `show_parse_events` | Emit parse pipeline events. | `false` |
| `format` | `text` or `json`. | `text` |

Unknown keys are logged as warnings but otherwise ignored, which allows experiment-specific toggles.

## `[svparser]`

Pass-through options for the SystemVerilog parser. All keys default to empty vectors or `true` to match the built-in sample config:

- `include_paths = []`
- `defines = []`
- `strip_comments = true`
- `ignore_include = true`
- `allow_incomplete = true`

## `[stages]`

| key | description | default |
| --- | --- | --- |
| `enabled` | Ordered list of stages to run. | `["raw_text","pp_text","cst","ast"]` |
| `required` | Subset that must succeed even if the transport would normally skip them. | `["raw_text","pp_text"]` |

Stage names map to `Stage::{RawText,PpText,Cst,Ast}`. Any rule that targets a disabled stage triggers a config error.
When no `[[rule]]` entries are present, sv-mint still runs but prints no diagnostics (the parser pipeline is not invoked in this mode).
If `stages.enabled` is empty, config loading fails. If `stages.required` is empty, sv-mint still treats `raw_text` and `pp_text` as required stages.

## `[[rule]]`

Every rule needs an `id`. The following keys are optional:

| key | description | default |
| --- | --- | --- |
| `script` | Python file implementing the rule. When omitted, sv-mint searches the plugin roots for `<id>.<stage>.py`. | _derived_ |
| `stage` | One of `raw_text`, `pp_text`, `cst`, or `ast`. If left out, sv-mint infers it from the filename suffix. | _inferred_ |
| `enabled` | Toggle to include or skip the rule. | `true` |
| `severity` | Overrides the severity reported by the script (`error`, `warning`, `info`; other values are rejected). | _script-provided_ |

`script` must end with `.py` for stage inference to succeed. When stage is omitted, the filename must also end with `.raw/.pp/.cst/.ast.py`.

Rules are grouped per stage at runtime. When every rule for a stage is disabled, that stage is logged as "no enabled rules" and skipped, and if the entire config lacks `[[rule]]` entries the CLI still parses inputs but emits no lint findings.

## `[transport]`

Controls JSON request/response size guards and how strictly to enforce them.

| key | description | default |
| --- | --- | --- |
| `max_request_bytes` | Maximum serialized size of a per-stage request. | `16777216` |
| `warn_margin_bytes` | Threshold before the request limit to emit warnings. | `1048576` |
| `max_response_bytes` | Maximum bytes allowed in a response payload. | `16777216` |
| `on_exceed` | `skip` or `error` for non-required stages. Required stages always error. | `skip` |
| `fail_ci_on_skip` | Upgrade skipped stages to CI failures. | `false` |

Values must be greater than zero, and `warn_margin_bytes` must not exceed `max_request_bytes`.

## Minimal example

```toml
# Only declare the bundled rules you want; everything else uses defaults.
[[rule]]
id = "module_no_port_wildcard"

[[rule]]
id = "decl_no_unused_var"
```

To target a custom rule directory, set `[plugin].root` and list your scripts with explicit `script` or by following the `<id>.<stage>.py` pattern so sv-mint can infer them automatically.
