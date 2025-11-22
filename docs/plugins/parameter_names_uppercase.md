# parameter_names_uppercase

## Script
- `plugins/parameter_names_uppercase.ast.py`

## Description
- Parameters must be UpperCamelCase

## Good

```systemverilog
module m # (
  parameter int MyParam = 1
);
endmodule
```

## Bad

```systemverilog
module m #(
  parameter int my_param = 1,
  parameter int MY_PARAM = 1
);
endmodule
```
