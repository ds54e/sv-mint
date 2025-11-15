# flow_wait_macro_required.py

- **Script**: `plugins/flow.wait_macro_required.raw.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`, `path`
- **Shared Helpers**: `plugins/lib/dv_text_ruleset.py`
- **Summary**: Raw `wait (cond)` usage must be replaced with `` `DV_WAIT``

## Details

### Trigger
Detects raw `wait (cond)` statements.
### Message
`` use `DV_WAIT(cond)` instead of raw wait ``
### Remediation
Wrap waits with the macro so watchdog timeouts are included.
### Good

```systemverilog
`DV_WAIT(req_done)
```

### Bad

```systemverilog
wait (req_done);
```
