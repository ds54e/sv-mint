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
- **Remediation**: Delete the variable, wire it into surrounding logic, or annotate intentional placeholders with inline comments that include `unused` (e.g., `` logic debug_shadow; // unused ``).
- **Notes**: Location data always comes from `sv-parser`, so when the declaration lives in an included file, inspect `Location.file`.
- **Good**:

```systemverilog
logic enable;
logic data_d;
logic data_q;

always_ff @(posedge clk_i) begin
  if (enable) data_q <= data_d;
end
```

```systemverilog
logic debug_shadow;  // unused (documented placeholder)
```

- **Bad**:

```systemverilog
logic enable;
logic data_d;
logic debug_shadow;  // never read or written
```

- **Additional Tips**: Only the declaration line is scanned for `unused`, so keep the inline note next to the symbol. Naming placeholders `*_unused` (and still referencing them) also communicates intent; for bundles of spare signals, group them into a single vector such as `logic [3:0] spare_signals = '0;`.
