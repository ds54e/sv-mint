# flow.no_disable_label

- **Script**: `plugins/flow.no_disable_label.cst.py`
- **Stage**: `cst` (`mode = inline`)
- **Key Inputs**: `cst_ir.tokens`, `line_starts`, `pp_text`
- **Summary**: `disable fork_label` is not portable

## Details

### Trigger
Walks CST `DisableStatement`; if it targets a label (anything other than `fork`), report it.
### Message
`` disable block label is not portable; use disable fork ``
### Remediation
Call `disable fork;` or rely on DV isolation helpers instead.
### Good

```systemverilog
disable fork;
```

### Bad

```systemverilog
disable worker_threads;
```
