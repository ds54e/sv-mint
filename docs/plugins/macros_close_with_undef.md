# macros_close_with_undef

- **Script**: `plugins/macros_close_with_undef.raw.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`, `path`
- **Summary**: Local `` `define`` entries must be `` `undef``’d in the same file

## Details

### Message
`` local macro <name> must be undefined before EOF ``
### Remediation
Add `` `undef`` once the macro is no longer needed.
### Good

```systemverilog
`define MY_MACRO(a) a
`undef MY_MACRO
```

### Bad

```systemverilog
`define MY_MACRO(a) a
```
