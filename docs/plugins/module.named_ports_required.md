# module.named_ports_required

- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Summary**: Require named `.port(signal)` connections

## Details
### Trigger
Detects instantiations that begin with positional arguments (no `.` inside the port list).
### Message
`` use named port connections instead of positional arguments ``
### Remediation
Rewrite as `.clk(clk)` style to remove ordering hazards.
### Notes
Formatting tools such as `verible-verilog-format --named-port-formatting` help during migrations.
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

`module.no_port_wildcard` is now enforced exclusively by `plugins/module.no_port_wildcard.cst.py` for precise diagnostics.
