# functions_args_have_direction

## Script
- `plugins/functions_args_have_direction.cst.py`

## Description
- Function arguments must specify a direction (`input`/`output`/`inout`/`ref`).
- Why: Explicit directions document intent and avoid default-input ambiguity.
## Good

```systemverilog
function logic f (input logic a);
  return a;
endfunction
```

## Bad

```systemverilog
function logic f (logic a);
  return a;
endfunction
```
