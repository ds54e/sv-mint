# one_module_per_file

- **Script**: `plugins/one_module_per_file.ast.py`
- **Stage**: `ast`
- **Summary**: Warn when a file declares more than one module

## Details

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
