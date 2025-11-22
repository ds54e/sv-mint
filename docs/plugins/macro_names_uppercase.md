# macro_names_uppercase

- **Script**: `plugins/macro_names_uppercase.raw.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Summary**: `define` names must be ALL_CAPS

## Details

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
