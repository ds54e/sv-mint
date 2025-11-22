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
### Notes
Falls back to text scanning when token data is unavailable.
### Good

```systemverilog
always_ff @(posedge clk_i or negedge rst_ni) begin
  if (!rst_ni) data_q <= '0;
  else data_q <= data_d;
end
```

### Bad

```systemverilog
always_ff @(posedge clk_i) begin
  data_q = data_d;  // blocking assignment inside always_ff
end
```
