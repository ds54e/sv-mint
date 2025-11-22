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
`define MY_MACRO
```systemverilog

### Bad

```systemverilog
`define my_macro
`define MyMacro
```systemverilog