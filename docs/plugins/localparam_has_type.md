# localparam_has_type

- **Script**: `plugins/localparam_has_type.cst.py`
- **Stage**: `cst`
- **Key Inputs**: `cst_ir.tokens`, `line_starts`, `pp_text`
- **Summary**: Localparams must declare an explicit data type

## Details

### Message
`` localparam must declare an explicit data type ``
### Remediation
Declare a data type for every `localparam`, including signedness and width as needed. A bit range alone (e.g., `localparam [7:0] DEPTH = 8;`) is not sufficient—include a type keyword like `int`/`logic`.
### Good

```systemverilog
`default_nettype none

module localparam_has_type_good;
  localparam int unsigned DEPTH = 16;
  localparam logic signed [3:0] OFFSET = -1;
  logic [DEPTH-1:0] data;
  assign data = {DEPTH{1'b0}} + OFFSET;
endmodule

`default_nettype wire
```systemverilog

### Bad

```systemverilog
`default_nettype none

module localparam_missing_type;
  localparam DEPTH = 16;
  logic [DEPTH-1:0] payload;
  assign payload = {DEPTH{1'b0}};
endmodule

`default_nettype wire
```systemverilog