# macros_use_module_prefix

- **Script**: `plugins/macros_use_module_prefix.raw.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`, `path`
- **Summary**: Module-local macros must be prefixed with the module name

## Details

### Message
`` module-local macros must be prefixed with MODULE_NAME_ ``
### Remediation
Rename macros to `FOO_CFG_*` if they live inside `module foo`.
### Good

```systemverilog
module my_module;
  `define MY_MODULE_MACRO
endmodule
```

### Bad

```systemverilog
module my_module;
  `define MACRO
endmodule
```
