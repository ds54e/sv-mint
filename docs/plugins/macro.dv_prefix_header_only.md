# macro.dv_prefix_header_only

- **Stage**: `raw_text`
- **Key Inputs**: `text`, `path`
- **Shared Helpers**: `plugins/lib/dv_text_ruleset.py`
- **Summary**: `DV_*` macros belong only in shared `_macros.svh` headers

## Details

### Trigger
Flags `DV_*` macros defined outside shared `_macros.svh` headers.
### Message
`` DV_* macros must live in shared macro headers ``
### Remediation
Move the macro into the common header or rename it without the `DV_` prefix.
### Good

```systemverilog
// shared_macros.svh
`define DV_RAL_POKE(addr, data) \
  `uvm_info(`gfn, {"poke:", addr}, UVM_HIGH)
```

### Bad

```systemverilog
// inside a test .sv
`define DV_RAL_POKE(addr, data) $display(addr, data);
```
