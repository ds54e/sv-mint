# check.dv_macro_required.raw.py

- **Stage**: `raw_text`
- **Key Inputs**: `text`, `path`
- **Shared Helpers**: `plugins/lib/dv_text_ruleset.py`
- **Summary**: Comparison-based checks must use `DV_CHECK_*` macros

## Details

### Trigger
Finds `if (lhs != rhs) uvm_error(...)` style comparisons that omit `DV_CHECK_*`.
### Message
`` use DV_CHECK_* macros for comparison-based checks ``
### Remediation
Replace manual comparisons with `DV_CHECK_EQ`, `DV_CHECK_NE`, etc.
### Good

```systemverilog
`DV_CHECK_EQ(exp_data, act_data)
```

### Bad

```systemverilog
if (exp_data != act_data) begin
  uvm_error(`gfn, "Mismatch");
end
```
