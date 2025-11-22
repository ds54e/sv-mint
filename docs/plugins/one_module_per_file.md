# one_module_per_file

- **Script**: `plugins/one_module_per_file.ast.py`
- **Stage**: `ast`
- **Summary**: Warn when a file declares more than one module

## Details

### Good

```systemverilog
module m;
endmodule
```

### Bad

```systemverilog
module m1;
endmodule

module m2;
endmodule
```
