# naming_var_lower_snake

- **Script**: `plugins/naming_var_lower_snake.ast.py`
- **Stage**: `ast`
- **Summary**: Warn when variable names are not lower_snake_case (letters, digits, `_`, `$` allowed)

## Details

### Trigger
Checks AST symbol table for variables whose names do not match `^[a-z][a-z0-9_$]*$`.

### Message
`` var names should use lower_snake_case (letters, digits, _, $ allowed): <name> ``

### Remediation
Rename variables to lower_snake_case, using `_` to separate words; `$` is permitted if needed.

### Good

```systemverilog
logic good_var;
logic dollar_var;
```

### Bad

```systemverilog
logic BadVar;
logic mixedCase;
```
