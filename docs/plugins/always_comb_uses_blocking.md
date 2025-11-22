# always_comb_uses_blocking

- **Script**: `plugins/always_comb_uses_blocking.cst.py`
- **Stage**: `cst`
- **Key Inputs**: `cst_ir.tokens`, `tok_kind_table`, `line_starts`, `pp_text`
- **Summary**: Ban non-blocking assignments (`<=`) inside `always_comb`

## Details

### Message
`` nonblocking '<=' inside always_comb ``
### Remediation
Use blocking `=` inside combinational logic; if state is required, move the logic to `always_ff`.
### Good

```systemverilog
always_comb begin
  result_d = a_i ^ b_i;  // blocking assignments only
end
```

### Bad

```systemverilog
always_comb begin
  result_q <= a_i ^ b_i;  // violates combinational semantics
end
```
