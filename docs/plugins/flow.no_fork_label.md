# flow.no_fork_label

- **Script**: `plugins/flow.no_fork_label.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`, `path`
- **Shared Helpers**: `plugins/lib/dv_text_ruleset.py`
- **Summary**: Forbid labeled `fork : label` syntax

## Details

### Trigger
Looks for `fork : label` syntax.
### Message
`` fork blocks must not be labeled ``
### Remediation
Use unlabeled `fork ... join` blocks or isolation helpers.
### Good

```systemverilog
fork
  do_task();
join_none
```

### Bad

```systemverilog
fork : worker_threads
  do_task();
join
```
