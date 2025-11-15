# log_allowed_verbosity.py

- **Script**: `plugins/log.allowed_verbosity.raw.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`, `path`
- **Shared Helpers**: `plugins/lib/dv_text_ruleset.py`
- **Rule**:
  - ``log.allowed_verbosity`` (warning): `uvm_*` macros must use UVM_LOW/MEDIUM/HIGH/DEBUG

## Rule Details

### `log.allowed_verbosity`
#### Trigger
Warns when the verbosity argument is a numeric literal or custom value.
#### Message
`` verbosity must be UVM_LOW/MEDIUM/HIGH/DEBUG ``
#### Remediation
Stick to the canonical verbosity constants.
#### Good

```systemverilog
uvm_info(`gfn, "Packet received", UVM_HIGH);
```

#### Bad

```systemverilog
uvm_info(`gfn, "Packet received", 700);
```
