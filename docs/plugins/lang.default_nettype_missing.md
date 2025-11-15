# lang.default_nettype_missing

- **Script**: `plugins/lang.default_nettype_missing.raw.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Shared Helpers**: `plugins/lib/default_nettype_ruleset.py`
- **Summary**: Require `` `default_nettype none`` in every file

## Details

Flags files that never declare `` `default_nettype``. The DVCodingStyle guidance (and most RTL style guides) expects `` `default_nettype none`` so that misspelled nets do not silently become implicit wires.
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
