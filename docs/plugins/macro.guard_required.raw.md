# macro.guard_required.raw.py

- **Stage**: `raw_text`
- **Key Inputs**: `text`, `path`
- **Shared Helpers**: `plugins/lib/dv_text_ruleset.py`
- **Summary**: Macros in global `_macros.svh` headers need `` `ifndef`` guards

## Details

### Trigger
Ensures `_macros.svh` files wrap each `define` in `` `ifndef`` guards.
### Message
`` macro headers must wrap definitions with `ifndef/`define/`endif ``
### Remediation
Add guards so re-including the header is safe.
### Good

```systemverilog
`ifndef FOO_MACROS_SVH
`define FOO_MACROS_SVH
`define FOO_CLR(req) req.clear()
`endif
```

### Bad

```systemverilog
`define FOO_CLR(req) req.clear()  // unguarded in shared header
```
