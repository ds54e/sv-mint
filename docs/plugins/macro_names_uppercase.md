# macro_names_uppercase

- **Script**: `plugins/macro_names_uppercase.raw.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Summary**: `define` names must be ALL_CAPS

## Details

### Good

```systemverilog
`define MY_MACRO
```

### Bad

```systemverilog
`define my_macro
`define MyMacro
```
