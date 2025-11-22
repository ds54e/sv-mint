# default_nettype_begins_with_none

- **Script**: `plugins/default_nettype_begins_with_none.raw.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Shared Helpers**: `plugins/lib/default_nettype_ruleset.py`
- **Summary**: Require `` `default_nettype none`` in every file

## Details

Flags files that never declare `` `default_nettype``. Requiring `` `default_nettype none`` ensures misspelled nets do not silently become implicit wires.
### Good

```systemverilog
`default_nettype none

module foo;
endmodule

`default_nettype wire
```

### Bad

```systemverilog
module foo;
endmodule  // no `default_nettype directive
```
