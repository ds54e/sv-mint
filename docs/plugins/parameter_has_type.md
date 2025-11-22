# parameter_has_type

- **Script**: `plugins/parameter_has_type.cst.py`
- **Stage**: `cst`
- **Key Inputs**: `cst_ir.tokens`, `line_starts`, `pp_text`
- **Summary**: Parameters must declare an explicit data type

## Details

### Message
`` parameter must declare an explicit data type ``
### Remediation
Declare an explicit data type for every `parameter`.
### Good

```systemverilog
module m #(
  parameter int unsigned MyParam1 = 1,
  parameter real MyParam2 = 1.0
);
endmodule
```

### Bad

```systemverilog
module m #(
  parameter MyParam1 = 1,
  parameter MyParam2 = 1.0
);
endmodule
```
