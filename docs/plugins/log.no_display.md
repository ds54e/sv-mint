# log.no_display

- **Script**: `plugins/log.no_display.raw.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`, `path`
- **Shared Helpers**: `plugins/lib/dv_text_ruleset.py`
- **Summary**: Forbid `$display` in DV code

## Details

### Trigger
Looks for `$display` within DV sources.
### Message
`` use uvm_* logging macros instead of $display ``
### Remediation
Replace `$display` with `uvm_info` and friends.
### Good

```systemverilog
uvm_info(`gfn, $sformatf("value=%0d", value_q), UVM_LOW);
```

### Bad

```systemverilog
$display("value=%0d", value_q);
```
