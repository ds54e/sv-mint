# ports_not_left_unused

- **Script**: `plugins/ports_not_left_unused.ast.py`
- **Stage**: `ast`
- **Key Inputs**: `ports` list plus `refs` read/write counts
- **Summary**: Warn when module ports are never read or written inside the module body

## Details

### Notes
- Implicit connections (`.*`, `.foo` shorthand) are not elaborated; they will be counted as unused.
- If the declaration line contains a comment with the words `used` or `reserved` (case-insensitive), the warning is suppressed.
### Good

```systemverilog
module m (
  input logic a,
  input logic b1, // reserved
  input logic b2, // used
  output logic c
);
  assign c = a;
endmodule
```

### Bad

```systemverilog
module m (
  input logic a,
  output logic b
);
  assign b = 1'b1;
endmodule
```
