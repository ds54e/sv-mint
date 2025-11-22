# module_names_lower_snake

- **Script**: `plugins/module_names_lower_snake.ast.py`
- **Stage**: `ast`
- **Key Inputs**: `decls`, `symbols`, `ports`
- **Summary**: Modules must use lower_snake_case

## Details

### Message
`` module <name> must use lower_snake_case ``
### Good

```systemverilog
module my_module;
endmodule
```

### Bad

```systemverilog
module MyModule;
endmodule

module MY_MODULE;
endmodule
```
