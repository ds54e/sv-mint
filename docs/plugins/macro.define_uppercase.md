# macro.define_uppercase

- **Script**: `plugins/macro.define_uppercase.raw.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Summary**: `define` names must be ALL_CAPS

## Details

### Trigger
Scans each `` `define NAME``; if `NAME` is not ALL_CAPS (`^[A-Z][A-Z0-9_]*$`), a warning is emitted.

### Message
`` `define <name> should use ALL_CAPS ``

### Remediation
Rename macros to ALL_CAPS (e.g., `` `define MY_MACRO 1``).

### Good

```systemverilog
`define FOO_BAR 1
```

### Bad

```systemverilog
`define fooBar 1
```
