# sensitivity_list_uses_commas

- **Script**: `plugins/sensitivity_list_uses_commas.cst.py`
- **Stage**: `cst`
- **Key Inputs**: `cst_ir.tokens`, `line_starts`, `pp_text`
- **Summary**: Prefer comma-separated sensitivity lists instead of `or`

## Details

### Message
`` use ',' separators in sensitivity lists instead of 'or' ``
### Remediation
Write event controls as `@(posedge clk, negedge rst_ni)` rather than `@(posedge clk or negedge rst_ni)` for portability and clarity.
### Good

```systemverilog
always_ff @(posedge clk_i, negedge rst_ni) begin
  if (!rst_ni) data_q <= '0;
  else data_q <= data_d;
end
```

### Bad

```systemverilog
always_ff @(posedge clk_i or negedge rst_ni) begin
  if (!rst_ni) data_q <= '0;
  else data_q <= data_d;
end
```
