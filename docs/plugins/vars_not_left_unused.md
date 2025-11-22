# vars_not_left_unused

- **Script**: `plugins/vars_not_left_unused.ast.py`
- **Stage**: `ast`
- **Key Inputs**: `symbols` entries with `class == var`
- **Summary**: Warn about variable declarations that are never referenced

## Details

### Message
`` unused var <module>.<name> ``
### Remediation
Delete the variable, wire it into surrounding logic, or annotate intentional placeholders with inline comments that include `unused` (e.g., `` logic debug_shadow; // unused ``).
### Good

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

### Bad

```systemverilog
logic enable;
logic data_d;
logic debug_shadow;  // never read or written
```
