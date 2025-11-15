# log.no_uvm_report_api

- **Script**: `plugins/log.no_uvm_report_api.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`, `path`
- **Shared Helpers**: `plugins/lib/dv_text_ruleset.py`
- **Summary**: Forbid `uvm_report_*` helpers and require the shorthand macros

## Details

### Trigger
Searches for `uvm_report_*` calls.
### Message
`` use uvm_info/error/fatal instead of uvm_report_* APIs ``
### Remediation
Switch to the shorthand macros (`uvm_info`, etc.).
### Good

```systemverilog
uvm_info(`gfn, "Starting sequence", UVM_MEDIUM);
```

### Bad

```systemverilog
uvm_report_info(`gfn, "Starting sequence", UVM_MEDIUM);
```
