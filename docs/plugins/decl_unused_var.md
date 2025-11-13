# decl_unused_var.py

- **Script**: `plugins/decl_unused_var.py`
- **Stage**: `ast`
- **Key Inputs**: `symbols` entries with `class == var`
- **Rule**:
  | Rule ID | Severity | Summary |
  | --- | --- | --- |
  | `decl.unused.var` | warning | Warn about variable declarations that are never referenced |

## Rule Details

### `decl.unused.var`
- **Trigger**: Looks for `var` symbols whose `read_count` and `write_count` both equal zero, reporting the declaration site.
- **Message**: `` unused var <module>.<name> ``
- **Remediation**: Delete the variable or wire it into logic. Avoid suppressing via tool-specific comments; disable the rule in config instead if necessary.
- **Notes**: Location data always comes from `sv-parser`, so when the declaration lives in an included file, inspect `Location.file`.
- **LowRISC Reference**: The guide discourages unused variables unless they are explicitly marked as `_unused`.
- **Good**:

```systemverilog
logic enable;
logic data_d;
logic data_q;

always_ff @(posedge clk_i) begin
  if (enable) data_q <= data_d;
end
```

- **Bad**:

```systemverilog
logic enable;
logic data_d;
logic debug_shadow;  // never read or written
```

- **Additional Tips**: Naming placeholders `*_unused` allows suppression via `allowlist.regex = ".*_unused$"`. To collect spare signals, declare a vector such as `logic [3:0] spare_signals = '0;` and tap bits explicitly when needed.
