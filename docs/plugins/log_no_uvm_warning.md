# log_no_uvm_warning.py

- **Script**: `plugins/log.no_uvm_warning.raw.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`, `path`
- **Shared Helpers**: `plugins/lib/dv_text_ruleset.py`
- **Rule**:
  - ``log.no_uvm_warning`` (warning): Ban `uvm_warning` in favor of `uvm_error`/`uvm_fatal`

## Rule Details

### `log.no_uvm_warning`
#### Trigger
Flags any use of `uvm_warning`.
#### Message
`` uvm_warning is banned; use uvm_error or uvm_fatal ``
#### Remediation
Upgrade warnings to `uvm_error` (or `uvm_fatal` when appropriate).
#### Good

```systemverilog
uvm_error(`gfn, "Timeout waiting for ack");
```

#### Bad

```systemverilog
uvm_warning(`gfn, "Timeout waiting for ack");
```
