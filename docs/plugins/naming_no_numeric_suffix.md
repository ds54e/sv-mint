# naming_no_numeric_suffix.py

- **Script**: `plugins/naming.no_numeric_suffix.ast.py`
- **Stage**: `ast`
- **Key Inputs**: `decls`, `symbols`, `ports`
- **Shared Helpers**: `plugins/lib/naming_ruleset.py`
- **Rule**:
  - ``naming.no_numeric_suffix`` (warning): Ban trailing `_42` numeric suffixes

## Rule Details

### `naming.no_numeric_suffix`
#### Trigger
Detects identifiers ending in `_<digits>`.
#### Message
`` <name> must not end with _<number> ``
#### Remediation
Use meaningful suffixes such as `_a/_b` or `_stage1/_stage2`, not raw numbers.
#### Good

```systemverilog
logic state_a, state_b;
```

#### Bad

```systemverilog
logic state_42;
```
