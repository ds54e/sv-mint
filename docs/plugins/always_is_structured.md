# always_is_structured

- **Script**: `plugins/always_is_structured.cst.py`
- **Stage**: `cst`
- **Key Inputs**: `cst_ir.tokens`, `line_starts`, `pp_text`
- **Summary**: Replace bare `always` with `always_ff`/`always_comb`/`always_latch`

## Details

### Message
`` use always_ff/always_comb/always_latch instead of bare always ``

### Remediation
Rewrite the process using one of the structured forms. For sequential logic, prefer `always_ff`; for combinational blocks, use `always_comb`; reserve `always_latch` for intentional latches.

### Good

```systemverilog
always_ff @(posedge clk_i, negedge rst_ni) begin
  if (!rst_ni) state_q <= IDLE;
  else state_q <= state_d;
end
```

```systemverilog
always_comb begin
  state_d = next_state;
end
```

### Bad

```systemverilog
always @(posedge clk_i, negedge rst_ni) begin
  if (!rst_ni) state_q <= IDLE;
  else state_q <= state_d;
end
```

```systemverilog
always @* begin
  state_d = next_state;
end
```
