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
localparam int unsigned DEPTH = 16;
localparam logic signed [3:0] OFFSET = -1;
```

### Bad

```systemverilog
localparam DEPTH = 16;           // missing type
localparam [7:0] OFFSET = 8'h0;  // range only
```
