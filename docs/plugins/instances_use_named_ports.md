# instances_use_named_ports

- **Script**: `plugins/instances_use_named_ports.cst.py`
- **Stage**: `cst`
- **Key Inputs**: `cst_ir.tokens`, `line_starts`, `pp_text`
- **Summary**: Require named `.port(signal)` connections

## Details
### Message
`` use named port connections instead of positional arguments ``
### Remediation
Rewrite as `.clk(clk)` style to remove ordering hazards.
### Good

```systemverilog
foo u_foo (
  .clk_i(clk_i),
  .rst_ni(rst_ni),
  .req_i(req_i),
  .gnt_o(gnt_o)
);
```

### Bad

```systemverilog
foo u_foo (clk_i, rst_ni, req_i, gnt_o);  // positional arguments
```

This rule inspects the CST for positional port connections and flags them so instantiations stay readable and ordering-safe.
