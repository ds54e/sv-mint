# ports_not_left_unused

- **Script**: `plugins/ports_not_left_unused.ast.py`
- **Stage**: `ast`
- **Key Inputs**: `ports` list plus `refs` read/write counts
- **Summary**: Warn when module ports are never read or written inside the module body

## Details

### Trigger
Ports with zero reads and zero writes in the same module, as measured from AST `refs`.
### Message
`` unused port <module>.<name> ``
### Remediation
Remove or route the port, or mark intentional placeholders with an inline `unused` comment on the declaration line.
### Notes
Only in-module references are counted; external instance connections do not satisfy the rule.
### Good

```systemverilog
module my_block(input logic a_i, output logic b_o);
  assign b_o = a_i;
endmodule
```

```systemverilog
module fixture(
  input  logic debug_i,  // unused
  output logic ready_o
);
  assign ready_o = 1'b0;
endmodule
```

### Bad

```systemverilog
module idle_block(input logic debug_i, output logic ready_o);
  assign ready_o = 1'b0;
endmodule
```
