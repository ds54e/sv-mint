# functions_explicit_arg_types

- **Script**: `plugins/functions_explicit_arg_types.cst.py`
- **Stage**: `cst`
- **Key Inputs**: `cst_ir.tokens`, `cst_ir.nodes`, `cst_ir.line_starts`
- **Summary**: Function arguments must declare explicit data types; implicit or identifier-only types are rejected.

## Details

### Message
`` function arguments must declare explicit data types ``
### Remediation
Annotate every argument with a full data type (width and signedness as needed), not just the identifier.
### Good

```systemverilog
function logic add(
  input logic a,
  input logic b
);
  return a + b;
endfunction
```

### Bad

```systemverilog
function logic add(
  input a,
  input b  // implicit 1-bit type
);
  return a + b;
endfunction
```
