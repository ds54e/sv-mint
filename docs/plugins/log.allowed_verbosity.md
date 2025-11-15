# log.allowed_verbosity

- **Script**: `plugins/log.allowed_verbosity.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`, `path`
- **Shared Helpers**: `plugins/lib/dv_text_ruleset.py`
- **Summary**: `uvm_*` macros must use UVM_LOW/MEDIUM/HIGH/DEBUG

## Details

### Trigger
Warns when the verbosity argument is a numeric literal or custom value.
### Message
`` verbosity must be UVM_LOW/MEDIUM/HIGH/DEBUG ``
### Remediation
Stick to the canonical verbosity constants.
### Good

```systemverilog
uvm_info(`gfn, "Packet received", UVM_HIGH);
```

### Bad

```systemverilog
uvm_info(`gfn, "Packet received", 700);
```
