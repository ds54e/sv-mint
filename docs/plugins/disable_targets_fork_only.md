# disable_targets_fork_only

- **Script**: `plugins/disable_targets_fork_only.cst.py`
- **Stage**: `cst`
- **Key Inputs**: `cst_ir.tokens`, `line_starts`, `pp_text`
- **Summary**: `disable fork_label` is not portable

## Details

### Message
`` disable block label is not portable; use disable fork ``
### Remediation
When disabling subprocesses, `disable fork` terminates all processes and `disable thread_label` disables a specific thread. `disable fork_label` is non-compliant with SystemVerilog-2017 (Sections 9.6.2/9.6.3) and is inconsistently supported, so use `disable fork;` instead.
### Good

```systemverilog
fork : fork_label
  begin : thread_a
    work_a();
  end
  begin : thread_b
    work_b();
  end
join_any
disable fork;
```

### Bad

```systemverilog
fork : fork_label
  begin : thread_a
    work_a();
  end
  begin : thread_b
    work_b();
  end
join_any
// Non-compliant: relies on disabling a fork label
disable fork_label;
```
