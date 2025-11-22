# var_names_lower_snake

- **Script**: `plugins/var_names_lower_snake.ast.py`
- **Stage**: `ast`
- **Summary**: Warn when variable names are not lower_snake_case (letters, digits, `_`, `$` allowed)

## Details

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
