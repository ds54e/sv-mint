# log.no_none_full

- **Script**: `plugins/log.no_none_full.raw.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`, `path`
- **Shared Helpers**: `plugins/lib/dv_text_ruleset.py`
- **Summary**: Ban `UVM_NONE` and `UVM_FULL` verbosity levels

## Details

### Trigger
Flags verbosity arguments equal to `UVM_NONE` or `UVM_FULL`.
### Message
`` use UVM_LOW/MEDIUM/HIGH/DEBUG verbosity levels ``
### Remediation
Choose one of the supported verbosity constants.
### Good

```systemverilog
uvm_info(`gfn, "Ping", UVM_LOW);
```

### Bad

```systemverilog
uvm_info(`gfn, "Ping", UVM_NONE);
```
