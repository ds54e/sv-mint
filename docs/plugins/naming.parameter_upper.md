# naming.parameter_upper

- **Script**: `plugins/naming.parameter_upper.py`
- **Stage**: `ast`
- **Key Inputs**: `decls`, `symbols`, `ports`
- **Shared Helpers**: `plugins/lib/naming_ruleset.py`
- **Summary**: Parameters must be UpperCamelCase

## Details

### Trigger
Flags `parameter` names that are not UpperCamelCase.
### Message
`` parameter <name> must use UpperCamelCase ``
### Remediation
Rename parameters to `DataWidth`, `NumAlerts`, etc.
### Good

```systemverilog
parameter int DataWidth = 32;
```

### Bad

```systemverilog
parameter int data_width = 32;
```
