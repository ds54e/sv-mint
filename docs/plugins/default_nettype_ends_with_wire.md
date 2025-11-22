# default_nettype_ends_with_wire

- **Script**: `plugins/default_nettype_ends_with_wire.cst.py`
- **Stage**: `cst`
- **Key Inputs**: `cst_ir.directives`, `line_starts`, `source_text`/`pp_text`
- **Summary**: Files must reset `default_nettype` back to `wire` near the end

## Details

### Good

```systemverilog
`default_nettype none

module m;
endmodule

`default_nettype wire
```

```systemverilog
module m;
endmodule
```

### Bad

```systemverilog
`default_nettype none

module m;
endmodule
```
