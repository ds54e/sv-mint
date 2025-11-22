# net_names_lower_snake

- **Script**: `plugins/net_names_lower_snake.ast.py`
- **Stage**: `ast`
- **Summary**: Warn when net names are not lower_snake_case (letters, digits, `_`, `$` allowed)

## Details

### Message
`` net names should use lower_snake_case (letters, digits, _, $ allowed): <name> ``

### Remediation
Rename nets to lower_snake_case, using `_` to separate words; `$` is permitted if needed.

### Good

```systemverilog
module m;
  wire my_net;
  wire my_net$abc;
endmodule
```systemverilog

### Bad

```systemverilog
module m;
  wire MyNet;
  wire MY_NET;
endmodule
```systemverilog