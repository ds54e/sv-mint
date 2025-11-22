# functions_explicit_return_type

## Script
- `plugins/functions_explicit_return_type.cst.py`

## Description
- Functions must declare an explicit return type.

## Good

```systemverilog
function automatic logic f (input logic a);
  return 1'b0;
endfunction
```

## Bad

```systemverilog
function automatic f (input logic a);
  return 1'b0;
endfunction
```
