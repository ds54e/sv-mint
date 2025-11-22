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
function automatic logic signed [3:0] acc_fn(
  input logic [1:0] a
);
  return a + 1'b1;
endfunction
```

### Bad

```systemverilog
function acc_fn(input a_i, input b_i);
  acc_fn = a_i + b_i;  // implicit return type
endfunction
```
