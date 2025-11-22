# parameter_has_type

- **Script**: `plugins/parameter_has_type.cst.py`
- **Stage**: `cst`
- **Key Inputs**: `cst_ir.tokens`, `line_starts`, `pp_text`
- **Summary**: Parameters must declare an explicit data type

## Details

### Message
`` parameter must declare an explicit data type ``
### Remediation
Declare an explicit data type for every `parameter`; a bit range alone is not sufficient. Localparams are covered by `localparam_has_type`.
### Good

```systemverilog
`default_nettype none

module parameter_has_type_good;
  parameter int WIDTH = 4;
  parameter signed [3:0] OFFSET = 0;
  parameter type T = int;
  localparam T VALUE = T'(WIDTH + OFFSET);
  logic [WIDTH-1:0] data;
  assign data = VALUE[WIDTH-1:0];
endmodule

`default_nettype wire
```

### Bad

```systemverilog
`default_nettype none

module parameter_missing_type;
  parameter WIDTH = 4;
endmodule

`default_nettype wire
```
