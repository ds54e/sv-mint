# module.name_matches_file

- **Script**: `plugins/module.name_matches_file.ast.py`
- **Stage**: `ast`
- **Summary**: Warn when the module or package name does not match the file name

## Details

### Trigger
- Module declarations whose name differs from the filename (sans extension).
- Package declarations whose name differs from the filename.

### Message
`` module name <name> should match file name <stem> ``
or
`` package name <name> should match file name <stem> ``

### Remediation
Rename the module/package or the file so they match (one module/package per file).

### Good

`module_filename_match_ok.sv`
```systemverilog
module module_filename_match_ok;
endmodule
```

### Bad

`module_filename_mismatch.sv`
```systemverilog
module wrong_name;
endmodule
```
