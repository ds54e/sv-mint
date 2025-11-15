# flow_spinwait_macro_required.py

- **Script**: `plugins/flow.spinwait_macro_required.raw.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`, `path`
- **Shared Helpers**: `plugins/lib/dv_text_ruleset.py`
- **Summary**: `while` polling loops must live inside `` `DV_SPINWAIT``

## Details

### Trigger
Flags `while` polling loops outside of `` `DV_SPINWAIT``.
### Message
`` polling loops must use `DV_SPINWAIT``
### Remediation
Wrap loops with the macro or move them into `DV_SPINWAIT`.
### Good

```systemverilog
`DV_SPINWAIT(req_done)
```

### Bad

```systemverilog
while (!req_done) begin
  #10ns;
end
```
