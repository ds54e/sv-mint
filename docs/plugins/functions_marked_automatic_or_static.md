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
function automatic int add(input int a, input int b);
  return a + b;
endfunction
```

### Bad

```systemverilog
function int add(input int a, input int b);
  return a + b;
endfunction
```
