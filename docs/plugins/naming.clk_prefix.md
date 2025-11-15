# naming.clk_prefix

- **Script**: `plugins/naming.clk_prefix.ast.py`
- **Stage**: `ast`
- **Key Inputs**: `decls`, `symbols`, `ports`
- **Shared Helpers**: `plugins/lib/naming_ruleset.py`
- **Summary**: Clock names must start with `clk`

## Details

### Trigger
Requires clock ports to start with `clk`.
### Message
`` clock port <name> must start with 'clk' ``
### Remediation
Rename to `clk_<domain>_<suffix>`.
### Good

```systemverilog
input logic clk_core_i;
```

### Bad

```systemverilog
input logic core_clk_i;
```
