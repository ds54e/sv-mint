# localparam_names_uppercase

- **Script**: `plugins/localparam_names_uppercase.ast.py`
- **Stage**: `ast`
- **Summary**: Warn when `localparam` names are not UpperCamelCase or ALL_CAPS

## Details

### Message
`` localparam <name> should use UpperCamelCase or ALL_CAPS ``

### Remediation
Rename localparams to follow UpperCamelCase (e.g., `WidthParam`) or ALL_CAPS (e.g., `BUS_WIDTH`).

### Good

```systemverilog
module m #(
  localparam int MyParam = 1,
  localparam int MY_CONST = 1
);
endmodule
```

### Bad

```systemverilog
module m #(
  localparam int my_const = 1
);
endmodule
```
