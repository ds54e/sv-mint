# macro.module_prefix

- **Script**: `plugins/macro.module_prefix.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`, `path`
- **Shared Helpers**: `plugins/lib/dv_text_ruleset.py`
- **Summary**: Module-local macros must be prefixed with the module name

## Details

### Trigger
Ensures macros defined inside modules start with the module name in uppercase.
### Message
`` module-local macros must be prefixed with MODULE_NAME_ ``
### Remediation
Rename macros to `FOO_CFG_*` if they live inside `module foo`.
### Good

```systemverilog
module foo;
  `define FOO_SET_CFG(val) cfg_q = (val)
endmodule
```

### Bad

```systemverilog
module foo;
  `define SET_CFG(val) cfg_q = (val);  // missing FOO_ prefix
endmodule
```
