# naming.no_numeric_suffix

- **Script**: `plugins/naming.no_numeric_suffix.py`
- **Stage**: `ast`
- **Key Inputs**: `decls`, `symbols`, `ports`
- **Shared Helpers**: `plugins/lib/naming_ruleset.py`
- **Summary**: Ban trailing `_42` numeric suffixes

## Details

### Trigger
Detects identifiers ending in `_<digits>`.
### Message
`` <name> must not end with _<number> ``
### Remediation
Use meaningful suffixes such as `_a/_b` or `_stage1/_stage2`, not raw numbers.
### Good

```systemverilog
logic state_a, state_b;
```

### Bad

```systemverilog
logic state_42;
```
