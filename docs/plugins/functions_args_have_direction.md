# functions_args_have_direction

- **Script**: `plugins/functions_args_have_direction.cst.py`
- **Stage**: `cst`
- **Key Inputs**: `cst_ir.tokens`, `cst_ir.nodes`, `cst_ir.line_starts`
- **Summary**: Function arguments must specify a direction (`input`/`output`/`inout`/`ref`); omitted directions are rejected.

## Details

### Message
`` function arguments must specify direction (input/output/inout/ref) ``
### Remediation
Add an explicit direction to every function argument.
### Good

```systemverilog
function logic add(
  input logic a,
  output logic sum_o
);
  sum_o = a;
  return sum_o;
endfunction
```

### Bad

```systemverilog
function logic add(
  input logic a,
  b  // missing direction
);
  return a + b;
endfunction
```
