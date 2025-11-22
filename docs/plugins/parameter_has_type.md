# parameter_has_type

## Script
- `plugins/parameter_has_type.cst.py`

## Description
- Parameters must declare an explicit data type

## Good

```systemverilog
module m #(
  parameter int unsigned MyParam1 = 1,
  parameter real MyParam2 = 1.0
);
endmodule
```

## Bad

```systemverilog
module m #(
  parameter MyParam1 = 1,
  parameter MyParam2 = 1.0
);
endmodule
```
