# disable_targets_fork_only

- **Script**: `plugins/disable_targets_fork_only.cst.py`
- **Stage**: `cst` (`mode = inline`)
- **Key Inputs**: `cst_ir.tokens`, `line_starts`, `pp_text`
- **Summary**: `disable fork_label` is not portable

## Details

### Trigger
Walks CST `DisableStatement`; if it targets a label (anything other than `fork`), report it.
### Message
`` disable block label is not portable; use disable fork ``
### Remediation
When disabling subprocesses, `disable fork` terminates all processes and `disable thread_label` disables a specific thread. `disable fork_label` is non-compliant with SystemVerilog-2017 (Sections 9.6.2/9.6.3) and is inconsistently supported, so use `disable fork;` instead.
### Good

```systemverilog
disable fork;
```

### Bad

```systemverilog
disable worker_threads;
```
