# naming_net_lower_snake

- **Script**: `plugins/naming_net_lower_snake.ast.py`
- **Stage**: `ast`
- **Summary**: Warn when net names are not lower_snake_case (letters, digits, `_`, `$` allowed)

## Details

### Trigger
Checks AST symbol table for nets whose names do not match `^[a-z][a-z0-9_$]*$`.

### Message
`` net names should use lower_snake_case (letters, digits, _, $ allowed): <name> ``

### Remediation
Rename nets to lower_snake_case, using `_` to separate words; `$` is permitted if needed.

### Good

```systemverilog
wire good_name;
wire dollar$name;
```

### Bad

```systemverilog
wire BadName;
wire mixedCaseSig;
```
