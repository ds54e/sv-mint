# macro.no_local_guard.raw.py

- **Stage**: `raw_text`
- **Key Inputs**: `text`, `path`
- **Shared Helpers**: `plugins/lib/dv_text_ruleset.py`
- **Summary**: Local macros must not use `` `ifndef`` guards

## Details

### Trigger
Warns when source files (non-header) wrap local macros inside `` `ifndef``.
### Message
`` local macros must not use `ifndef guards ``
### Remediation
Remove the guard so redefinition errors surface immediately.
### Good

```systemverilog
`define _LOCAL_DEBUG(msg) \
  uvm_info(`gfn, msg, UVM_LOW)
`undef _LOCAL_DEBUG
```

### Bad

```systemverilog
`ifndef _LOCAL_DEBUG
`define _LOCAL_DEBUG(msg) $display(msg);
`endif
```
