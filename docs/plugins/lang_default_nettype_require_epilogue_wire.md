# lang_default_nettype_require_epilogue_wire

- **Script**: `plugins/lang_default_nettype_require_epilogue_wire.raw.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Shared Helpers**: `plugins/lib/default_nettype_ruleset.py`
- **Summary**: Files must reset `default_nettype` back to `wire` near the end

## Details

Because `default_nettype` stays in effect for all subsequent compilation units, each file must reset it back to `wire` at the end so other sources aren’t accidentally processed with `none`. This rule looks at the last `default_nettype` directive and warns when it doesn’t restore `wire`.
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
