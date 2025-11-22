# parameter_has_type

- **Script**: `plugins/parameter_has_type.cst.py`
- **Stage**: `cst` (`mode = inline`)
- **Key Inputs**: `cst_ir.tokens`, `line_starts`, `pp_text`
- **Summary**: Parameters must declare an explicit data type

## Details

### Message
`` parameter must declare an explicit data type ``
### Remediation
Declare a data type for every `parameter`, such as `parameter int WIDTH = 4;`, `parameter signed [3:0] OFFSET = 0;`, or `parameter type T = int;`.
### Good

```systemverilog
parameter int WIDTH = 4;
parameter signed [3:0] OFFSET = 0;
parameter type T = int;
```

### Bad

```systemverilog
parameter WIDTH = 4;  // missing explicit type
```
