# one_module_per_file

## Script
- `plugins/one_module_per_file.ast.py`

## Description
- Warn when a file declares more than one module
- Why: Single-module files are easier to index, review, and build incrementally.
## Good

```systemverilog
module m;
endmodule
```

## Bad

```systemverilog
module m1;
endmodule

module m2;
endmodule
```
