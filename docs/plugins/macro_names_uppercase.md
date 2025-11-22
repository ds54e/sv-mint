# macro_names_uppercase

## Script
- `plugins/macro_names_uppercase.raw.py`

## Description
- `define` names must be ALL_CAPS
- ALL_CAPS visually separates macros from signals.
## Good

```systemverilog
`define MY_MACRO
```

## Bad

```systemverilog
`define my_macro
`define MyMacro
```
