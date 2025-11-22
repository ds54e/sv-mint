# functions_marked_automatic_or_static

## Script
- `plugins/functions_marked_automatic_or_static.cst.py`

## Description
- Functions in modules/interfaces/packages must declare `automatic` or `static`
- Why: Explicit lifetime avoids unintended shared state or tool-dependent storage.
## Good

```systemverilog
function automatic logic f1 (input logic a);
  return 1'b0;
endfunction

function static logic f2 (input logic a);
  return 1'b0;
endfunction
```

## Bad

```systemverilog
function logic f (input logic a);
  return 1'b0;
endfunction
```
