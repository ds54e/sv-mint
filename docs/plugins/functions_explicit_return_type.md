# functions_explicit_return_type

- **Script**: `plugins/functions_explicit_return_type.cst.py`
- **Stage**: `cst`
- **Key Inputs**: `cst_ir.tokens`, `cst_ir.nodes`, `cst_ir.line_starts`
- **Summary**: Functions must declare an explicit return type; implicit return types are rejected.

## Details

### Message
`` function must declare an explicit return type ``
### Remediation
Annotate the function header with a concrete return type (including width/signedness when needed).
### Good

```systemverilog
function automatic logic f (input logic a);
  return 1'b0;
endfunction
```

### Bad

```systemverilog
function automatic f (input logic a);
  return 1'b0;
endfunction
```
