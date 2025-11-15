# log.uvm_arg_macro

- **Stage**: `raw_text`
- **Key Inputs**: `text`, `path`
- **Shared Helpers**: `plugins/lib/dv_text_ruleset.py`
- **Summary**: `uvm_{info,error,fatal}` must use `` `gfn``/`` `gtn`` tags

## Details

### Trigger
Ensures the first argument to `uvm_info/error/fatal` is `` `gfn`` or `` `gtn``.
### Message
`` first argument to uvm_* must be `gfn or `gtn ``
### Remediation
Replace literal strings with the standard macros for hierarchy tags.
### Good

```systemverilog
uvm_info(`gfn, "DMA started", UVM_LOW);
```

### Bad

```systemverilog
uvm_info("dma", "DMA started", UVM_LOW);
```
