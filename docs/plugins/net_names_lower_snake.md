# net_names_lower_snake

- **Script**: `plugins/net_names_lower_snake.ast.py`
- **Stage**: `ast`
- **Summary**: Warn when net names are not lower_snake_case (letters, digits, `_`, `$` allowed)

## Details

### Good

```systemverilog
module m;
  wire my_net;
  wire my_net$abc;
endmodule
```

### Bad

```systemverilog
module m;
  wire MyNet;
  wire MY_NET;
endmodule
```
