# functions_marked_automatic_or_static

- **Script**: `plugins/functions_marked_automatic_or_static.cst.py`
- **Stage**: `cst`
- **Key Inputs**: `cst_ir.tokens`, `line_starts`, `pp_text`
- **Summary**: Functions in modules/interfaces/packages must declare `automatic` or `static`

## Details

### Message
`` functions in packages/modules/interfaces must declare automatic or static ``

### Remediation
Add `automatic` (recommended) or `static` to the function declaration inside modules, interfaces, packages, or programs.

### Good

```systemverilog
function automatic logic f1 (input logic a);
  return 1'b0;
endfunction

function static logic f2 (input logic a);
  return 1'b0;
endfunction
```

### Bad

```systemverilog
function logic f (input logic a);
  return 1'b0;
endfunction
```
