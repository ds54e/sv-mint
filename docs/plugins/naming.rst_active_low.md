# naming.rst_active_low

- **Script**: `plugins/naming.rst_active_low.ast.py`
- **Stage**: `ast`
- **Key Inputs**: `decls`, `symbols`, `ports`
- **Shared Helpers**: `plugins/lib/naming_ruleset.py`
- **Summary**: Reset names must end with `_n` (or `_ni/_no/_nio`)

## Details

### Trigger
Ensures reset names end in `_n`, `_ni`, `_no`, or `_nio`.
### Message
`` reset <name> must use active-low suffix `_n` ``
### Remediation
Rename resets to `rst_ni`, `rst_no`, etc.
### Good

```systemverilog
input logic rst_ni;
```

### Bad

```systemverilog
input logic rst_i;
```
