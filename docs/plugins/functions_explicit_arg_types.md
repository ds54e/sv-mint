# functions_explicit_arg_types

## Script
- `plugins/functions_explicit_arg_types.cst.py`

## Description
- Function arguments must declare explicit data types.
- Why: Implicit types hide width/signedness and can diverge across tools.
## Good

```systemverilog
function logic f (input logic a);
  return a;
endfunction
```

## Bad

```systemverilog
function logic f (input a);
  return a;
endfunction
```
