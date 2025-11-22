# default_nettype_ends_with_wire

- **Script**: `plugins/default_nettype_ends_with_wire.cst.py`
- **Stage**: `cst`
- **Key Inputs**: `cst_ir.directives`, `line_starts`, `source_text`/`pp_text`
- **Summary**: Files must reset `default_nettype` back to `wire` near the end

## Details

Because `default_nettype` stays in effect for all subsequent compilation units, each file that changes it must reset it back to `wire` at the end so other sources aren’t accidentally processed with `none`. The CST directives table is scanned for the last `default_nettype` directive; the rule warns when it doesn’t restore `wire`, and ignores files that never set `default_nettype`.
### Good

```systemverilog
`default_nettype none
module foo; endmodule
`default_nettype wire
```

### Bad

```systemverilog
`default_nettype none
module foo; endmodule
// missing reset to wire
```
