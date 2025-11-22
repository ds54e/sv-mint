# default_nettype_begins_with_none

- **Script**: `plugins/default_nettype_begins_with_none.cst.py`
- **Stage**: `cst`
- **Key Inputs**: `cst_ir.directives`, `line_starts`, `source_text`/`pp_text`
- **Summary**: Require `` `default_nettype none`` in every file

## Details

### Good

```systemverilog
`default_nettype none

module m;
endmodule

`default_nettype wire
```

### Bad

```systemverilog
module m;
endmodule
```
