# localparam_names_uppercase

- **Script**: `plugins/localparam_names_uppercase.ast.py`
- **Stage**: `ast`
- **Summary**: Warn when `localparam` names are not ALL_CAPS

## Details

### Message
`` localparam <name> should use ALL_CAPS ``

### Remediation
Rename localparams to ALL_CAPS (e.g., `BUS_WIDTH`).

### Good

```systemverilog
localparam int BUS_WIDTH = 32;
```

### Bad

```systemverilog
localparam int bus_width = 32;
localparam int mixedCase_param = 16;
localparam int WidthParam = 16;
```
