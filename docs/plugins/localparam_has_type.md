# localparam_has_type

## Script
- `plugins/localparam_has_type.cst.py`

## Description
- Localparams must declare an explicit data type
- Why: Explicit types avoid default widths and make intent clear for compile-time constants.
## Good

```systemverilog
module m #(
  localparam int unsigned MyParam1 = 1,
  localparam real MyParam2 = 1.0
);
endmodule
```

## Bad

```systemverilog
module m #(
  localparam MyParam1 = 1,
  localparam MyParam2 = 1.0
);
endmodule
```
