# always_ff_uses_nonblocking

- **Script**: `plugins/always_ff_uses_nonblocking.cst.py`
- **Stage**: `cst`
- **Key Inputs**: `cst_ir.tokens`, `tok_kind_table`, `line_starts`, `pp_text`
- **Summary**: Ban blocking `=` assignments inside `always_ff`

## Details

### Message
`` blocking '=' inside always_ff ``
### Remediation
Use non-blocking `<=` for sequential logic or refactor the assignment into combinational logic.
### Good

```systemverilog
always_ff @(posedge clk_i) begin
  data_q <= data_d;  // non-blocking assignment inside always_ff
end
```

### Bad

```systemverilog
always_ff @(posedge clk_i) begin
  data_q = data_d;  // blocking assignment inside always_ff
end
```
