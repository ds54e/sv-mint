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
parameter int DataWidth = 32;
```

### Bad

```systemverilog
parameter int data_width = 32;
parameter int DATA_WIDTH = 32;
```
