# macros_close_with_undef

- **Script**: `plugins/macros_close_with_undef.raw.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`, `path`
- **Shared Helpers**: `plugins/lib/dv_text_ruleset.py`
- **Summary**: Local `` `define`` entries must be `` `undef``’d in the same file

## Details

### Message
`` local macro <name> must be undefined before EOF ``
### Remediation
Add `` `undef`` once the macro is no longer needed.
### Good

```systemverilog
`define _INC(x) ((x)+1)
assign data_o = `_INC(data_i);
`undef _INC
```

### Bad

```systemverilog
`define _INC(x) ((x)+1)
assign data_o = `_INC(data_i);  // leaks macro
```
