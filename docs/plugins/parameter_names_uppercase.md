# parameter_names_uppercase

- **Script**: `plugins/parameter_names_uppercase.ast.py`
- **Stage**: `ast`
- **Key Inputs**: `decls`, `symbols`, `ports`
- **Shared Helpers**: `plugins/lib/naming_ruleset.py`
- **Summary**: Parameters must be UpperCamelCase or ALL_CAPS

## Details

### Message
`` parameter <name> should use UpperCamelCase or ALL_CAPS ``
### Remediation
Rename parameters to `DataWidth`, `NumAlerts`, or `DATA_WIDTH`, etc.
### Good

```systemverilog
parameter int DataWidth = 32;
parameter int DATA_WIDTH = 32;
```

### Bad

```systemverilog
parameter int data_width = 32;
```
