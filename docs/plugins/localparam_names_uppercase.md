# localparam_names_uppercase

## Script
- `plugins/localparam_names_uppercase.ast.py`

## Description
- Warn when `localparam` names are not UpperCamelCase or ALL_CAPS

## Good

```systemverilog
module m #(
  localparam int MyParam = 1,
  localparam int MY_CONST = 1
);
endmodule
```

## Bad

```systemverilog
module m #(
  localparam int my_const = 1
);
endmodule
```
