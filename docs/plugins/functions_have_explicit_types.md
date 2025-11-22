# functions_have_explicit_types

- **Script**: `plugins/functions_have_explicit_types.cst.py`
- **Stage**: `cst`
- **Key Inputs**: `cst_ir.tokens`, `line_starts`, `pp_text`
- **Summary**: Functions must declare explicit return and argument data types

## Details

### Message
`` function must declare an explicit return type ``
`` function arguments must declare explicit data types ``
### Remediation
Annotate function return types (`function void foo;` or `function logic [3:0] foo;`) and provide data types for every argument (`input logic [7:0] data_i`). Do not rely on implicit types or range-only declarations.
### Good

```systemverilog
function automatic logic [7:0] acc_fn(input logic [7:0] a_i, input logic [7:0] b_i);
  acc_fn = a_i + b_i;
endfunction
```

### Bad

```systemverilog
function acc_fn(input a_i, input b_i);  // missing return and argument types
  acc_fn = a_i + b_i;
endfunction
```
