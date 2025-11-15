# flow.wait_fork_isolation

- **Script**: `plugins/flow.wait_fork_isolation.raw.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`, `path`
- **Shared Helpers**: `plugins/lib/dv_text_ruleset.py`
- **Summary**: `wait fork` must be replaced with isolation fork helpers

## Details

### Trigger
Reports `wait fork`.
### Message
`` wait fork is banned; use isolation helpers ``
### Remediation
Use watchdog-backed isolation helpers such as `DV_SPINWAIT`.
### Good

```systemverilog
`DV_SPINWAIT(wait_done);
```

### Bad

```systemverilog
wait fork;  // blocked until all child processes finish
```
