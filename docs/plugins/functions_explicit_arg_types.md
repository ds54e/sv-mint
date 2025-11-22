# functions_explicit_arg_types

- **Script**: `plugins/functions_explicit_arg_types.cst.py`
- **Stage**: `cst`
- **Key Inputs**: `cst_ir.tokens`, `cst_ir.nodes`, `cst_ir.line_starts`
- **Summary**: Function arguments must declare explicit data types.

## Details

### Good

```systemverilog
function logic f (input logic a);
  return a;
endfunction
```

### Bad

```systemverilog
function logic f (input a);
  return a;
endfunction
```
