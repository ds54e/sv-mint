# module_name_matches_filename

## Script
- `plugins/module_name_matches_filename.ast.py`

## Description
- Warn when the module or package name does not match the file name

## Good

`module_filename_match_ok.sv`
```systemverilog
module good;
endmodule
```

## Bad

`module_filename_mismatch.sv`
```systemverilog
module not_bad;
endmodule
```
