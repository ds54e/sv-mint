# localparam_has_type

- **Script**: `plugins/localparam_has_type.cst.py`
- **Stage**: `cst`
- **Key Inputs**: `cst_ir.tokens`, `line_starts`, `pp_text`
- **Summary**: Localparams must declare an explicit data type

## Details

### Message
`` localparam must declare an explicit data type ``
### Good

```systemverilog
module m #(
  localparam int unsigned MyParam1 = 1,
  localparam real MyParam2 = 1.0
);
endmodule
```

### Bad

```systemverilog
module m #(
  localparam MyParam1 = 1,
  localparam MyParam2 = 1.0
);
endmodule
```
