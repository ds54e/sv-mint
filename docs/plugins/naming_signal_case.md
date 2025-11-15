# naming.signal_case.ast.py

- **Stage**: `ast`
- **Key Inputs**: `decls`, `symbols`, `ports`
- **Shared Helpers**: `plugins/lib/naming_ruleset.py`
- **Summary**: Signals/variables must use lower_snake_case

## Details

### Trigger
Checks nets and variables for lower_snake_case identifiers.
### Message
`` signal <name> must use lower_snake_case ``
### Remediation
Rename `logic`/`wire`/`reg` identifiers to lowercase snake case.
### Good

```systemverilog
logic error_flag;
```

### Bad

```systemverilog
logic errorFlag;
```
