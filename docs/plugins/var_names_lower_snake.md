# var_names_lower_snake

- **Script**: `plugins/var_names_lower_snake.ast.py`
- **Stage**: `ast`
- **Summary**: Warn when variable names are not lower_snake_case (letters, digits, `_`, `$` allowed)

## Details

### Message
`` var names should use lower_snake_case (letters, digits, _, $ allowed): <name> ``

### Good

```systemverilog
module m;
  logic my_var;
  logic my_var$abc;
endmodule
```

### Bad

```systemverilog
module m;
  logic MyVar;
  logic MY_VAR;
endmodule
```
