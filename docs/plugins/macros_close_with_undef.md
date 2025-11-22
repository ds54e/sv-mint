# macros_close_with_undef

## Script
- `plugins/macros_close_with_undef.raw.py`

## Description
- Local `` `define`` entries must be `` `undef``’d in the same file

## Good

```systemverilog
`define MY_MACRO(a) a
`undef MY_MACRO
```

## Bad

```systemverilog
`define MY_MACRO(a) a
```
