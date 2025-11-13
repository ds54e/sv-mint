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

## 4. Debugging Tips
- Run `sv-mint --config ... path` with `logging.show_plugin_events = true` to see `PluginInvoke` / `PluginDone` entries and stage timings.
- Optional unit tests can call `check` directly via `pytest`, feeding JSON fixtures.
- Temporary prints should go to stderr; bump `logging.stderr_snippet_bytes` so the CLI captures them.

## 5. Quality and Operations
- Use `category.name` style rule IDs so users can search the README or user guide easily.
- Filter the AST/CST before heavy processing and avoid copying entire payloads.
- For custom project rules, create subdirectories under `plugins/` and reference absolute or relative paths from `ruleset.scripts`. Document every rule under `docs/plugins/<script_name>.md`.

## 6. Known Size and Time Limits
- Request JSON larger than 16 MB stops the stage (required stages error out). When handling large payloads, trim unused fields or summarize reports.
- `timeout_ms_per_file` covers all stages for the file, so no single rule should monopolize the budget.

Consult [docs/internal_spec.md](internal_spec.md) for pipeline, size-guard, and event-system details.
