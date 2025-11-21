# module.no_multiple_modules

- **Script**: `plugins/module.no_multiple_modules.ast.py`
- **Stage**: `ast`
- **Summary**: Warn when a file declares more than one module

## Details

### Trigger
Counts module declarations in the AST payload; if more than one exists, every module after the first is reported.

### Message
`` file must contain only one module declaration ``

### Remediation
Split modules into separate files so each file contains exactly one module declaration.

### Good

```systemverilog
module single;
endmodule
```

### Bad

```systemverilog
module foo;
endmodule

module bar;
endmodule
```
