# Plugin Author Guide

## 0. Audience
For engineers who write or modify sv-mint Python rules under `plugins/` without touching the Rust core.

## 1. Execution Model
```
Rust Pipeline --NDJSON--> plugins/lib/rule_host.py --Python import--> scripts/*.py
```
- Rust launches a single Python host process and imports all `ruleset.scripts` entries.
- Each stage request is a single JSON line such as `{"kind":"run_stage","stage":"ast","path":"...","payload":{...}}`.
- The host calls `module.check(req)` in load order, concatenates all returned violations, and streams them back to Rust.
- When Rust finishes it sends `{"kind":"shutdown"}`, prompting the host to exit.

## 2. `check(req)` Contract
### 2.1 Request Structure
```python
req = {
    "kind": "run_stage",
    "stage": "raw_text" | "pp_text" | "cst" | "ast",
    "path": "/abs/path/to/file.sv",
    "payload": <StagePayload>,
}
```
`payload` depends on the stage:

| Stage | Contents |
| --- | --- |
| `raw_text` | `{ "text": "..." }` (LF-normalized source) |
| `pp_text` | `{ "text": "...", "defines": [{"name","value"}] }` |
| `cst` | `{ "mode": "inline", "cst_ir": {...} }` or `{ "mode": "none", "has_cst": bool }` |
| `ast` | `AstSummary` with `decls`, `refs`, `symbols`, `assigns`, `ports`, `pp_text`, etc. |

See [docs/internal_spec.md](internal_spec.md) for `AstSummary` and `cst_ir` schemas.

### 2.2 Response Structure
`check` returns an array of violation dictionaries. `None` or an empty list means no findings.

```python
return [{
    "rule_id": "format.line_length",
    "severity": "warning",  # error / warning / info
    "message": "line exceeds 120 columns (134)",
    "location": {"line": 10, "col": 121, "end_line": 10, "end_col": 135}
}]
```
Locations are 1-based; `end_*` can be inclusive or exclusive because sv-mint prints the values verbatim.

### 2.3 Exceptions
Unhandled exceptions bubble up as `{ "type": "error" }` responses and fail the entire stage. Prefer returning a violation such as `sys.rule.internal` or shield risky sections with `try/except`.

### 2.4 Stage Payload Reference
Each stage has a predictable JSON layout. Prefer `.get()` accessors so your rule keeps working when upstream fields evolve.

| Stage | Key Fields | Notes |
| --- | --- | --- |
| `raw_text` | `text`, `bytes_len`, `line_ending` | `text` is LF-normalized, but `bytes_len` lets you reference the original file size for metrics. |
| `pp_text` | `text`, `defines[]`, `include_stack[]` | Reflects the preprocessor output; macro bodies and conditional paths are fully expanded. |
| `cst` | `mode`, `cst_ir.tokens`, `cst_ir.nodes`, `tok_kind_table`, `line_starts` | `mode: "none"` appears when parsing stops early; handle this by skipping the rule gracefully. |
| `ast` | `decls`, `refs`, `symbols`, `assigns`, `ports`, `pp_text`, `defines` | Collections include `loc` objects with `file` info so you can report includes accurately. |

`sv-mint` ships a developer-only flag `--dump-payload <stage>` (guarded behind `cfg(debug_assertions)`). When unavailable, add temporary logging inside `plugins/lib/rule_host.py` to inspect incoming payloads.

## 3. Skeleton Example
```python
from typing import Any, Dict, List

RULE_ID = "example.rule"


def check(req: Dict[str, Any]) -> List[Dict[str, Any]]:
    if req.get("stage") != "ast":
        return []
    payload = req.get("payload") or {}
    symbols = payload.get("symbols") or []
    violations = []
    for sym in symbols:
        if sym.get("class") == "var" and sym.get("name", "").startswith("tmp_"):
            loc = sym.get("loc") or {"line": 1, "col": 1, "end_line": 1, "end_col": 1}
            violations.append({
                "rule_id": RULE_ID,
                "severity": "warning",
                "message": f"temporary signal {sym.get('name')} must be removed",
                "location": loc,
            })
    return violations
```
`plugins/template_raw_text_rule.py` is another minimal template.

### 3.1 Location Helper
When synthesizing locations, mirror sv-mint’s format to keep `git diff` noise low:

```python
def make_loc(line, col, end_line=None, end_col=None, file=None):
    return {
        "line": line,
        "col": col,
        "end_line": end_line or line,
        "end_col": end_col or col + 1,
        "file": file,
    }
```

Prefer the parser-provided `loc` objects whenever available to avoid off-by-one errors with multibyte characters.

## 4. Debugging Tips
- Run `sv-mint --config ... path` with `logging.show_plugin_events = true` to see `PluginInvoke` / `PluginDone` entries and stage timings.
- Optional unit tests can call `check` directly via `pytest`, feeding JSON fixtures.
- Temporary prints should go to stderr; bump `logging.stderr_snippet_bytes` so the CLI captures them.

### 4.1 Local Reproduction Checklist
1. Add your script to `[ruleset.scripts]` in `sv-mint.toml`.
2. Create a failing SystemVerilog sample under `fixtures/`.
3. Run `cargo run -- --config sv-mint.toml fixtures/sample.sv`.
4. Set `logging.show_plugin_events = true` to inspect per-rule timings.
5. Review `target/tmp/<ts>/plugin-stderr.log` if the rule crashes.
6. Capture reference output (and plug it into `tests/cli_smoke.rs` if the rule is part of the default bundle).

## 5. Quality and Operations
- Use `category.name` style rule IDs so users can search the README or user guide easily.
- Filter the AST/CST before heavy processing and avoid copying entire payloads.
- For custom project rules, create subdirectories under `plugins/` and reference absolute or relative paths from `ruleset.scripts`. Document every rule under `docs/plugins/<script_name>.md`.

### 5.1 Configuration Hooks
- `ruleset.scripts.<name>.path`: absolute or repo-relative path to the plugin.
- `ruleset.scripts.<name>.stage`: `raw_text`, `pp_text`, `cst`, or `ast`. Defaults to `ast`.
- `ruleset.scripts.<name>.enabled`: feature-flag a rule without removing it from the config.
- `[ruleset.override]`: change severity per `rule_id`.
- `[[ruleset.allowlist]]`: suppress findings by `rule_id`, globbed `path`, or regex.

Document any non-default toggles inside the corresponding `docs/plugins/*.md` entry.

## 6. Known Size and Time Limits
- Request JSON larger than 16 MB stops the stage (required stages error out). When handling large payloads, trim unused fields or summarize reports.
- `timeout_ms_per_file` covers all stages for the file, so no single rule should monopolize the budget.

## 7. Testing and Release Checklist
- **Unit tests**: use `pytest` with recorded payload snippets.
- **Golden diagnostics**: check fixtures into `fixtures/` and wire them into `tests/cli_smoke.rs`.
- **Performance**: ensure the rule completes in <10 ms/file for typical inputs; paginate work for very large AST sets.
- **Determinism**: sort AST collections before iterating so output remains stable across Python versions.
- **User guidance**: craft actionable `message` strings (naming templates, fix hints, doc links).
- **Telemetry**: include counters (e.g., number of offending symbols) in `message` to help triage.

Consult [docs/internal_spec.md](internal_spec.md) for pipeline, size-guard, and event-system details.
