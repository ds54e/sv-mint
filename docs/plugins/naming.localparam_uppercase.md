# naming.localparam_uppercase

- **Script**: `plugins/naming.localparam_uppercase.ast.py`
- **Stage**: `ast`
- **Summary**: Warn when `localparam` names are not UpperCamelCase or ALL_CAPS

## Details

### Trigger
Checks AST declarations for `localparam`; if the name is not `UpperCamelCase` or `ALL_CAPS` (letters, digits, underscores allowed for ALL_CAPS), it reports a warning.

### Message
`` localparam <name> should use UpperCamelCase or ALL_CAPS ``

### Remediation
Rename localparams to follow UpperCamelCase (e.g., `WidthParam`) or ALL_CAPS (e.g., `BUS_WIDTH`).

### Good

```systemverilog
localparam int BUS_WIDTH = 32;
localparam int WidthParam = 16;
```

### Bad

```systemverilog
localparam int bus_width = 32;
localparam int mixedCase_param = 16;
```
