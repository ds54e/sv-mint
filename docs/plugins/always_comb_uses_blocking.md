# always_comb_uses_blocking

- **Script**: `plugins/always_comb_uses_blocking.cst.py`
- **Stage**: `cst`
- **Key Inputs**: `cst_ir.tokens`, `tok_kind_table`, `line_starts`, `pp_text`
- **Summary**: Ban non-blocking assignments (`<=`) inside `always_comb`

## Details

### Message
`` nonblocking '<=' inside always_comb ``
### Good

```systemverilog
module m;
  logic a;
  always_comb begin
    a = 1'b1;
  end
endmodule
```

### Bad

```systemverilog
module m;
  logic a;
  always_comb begin
    a <= 1'b1;
  end
endmodule
```
