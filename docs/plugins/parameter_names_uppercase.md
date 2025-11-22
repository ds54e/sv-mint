# parameter_names_uppercase

- **Script**: `plugins/parameter_names_uppercase.ast.py`
- **Stage**: `ast`
- **Key Inputs**: `decls`, `symbols`, `ports`
- **Summary**: Parameters must be UpperCamelCase

## Details

### Message
`` parameter <name> should use UpperCamelCase ``
### Remediation
Rename parameters to `DataWidth` or `NumAlerts`; ALL_CAPS is not allowed.
### Good

```systemverilog
module m # (
  parameter int MyParam = 1
);
endmodule
```

### Bad

```systemverilog
module m #(
  parameter int my_param = 1,
  parameter int MY_PARAM = 1
);
endmodule
```
