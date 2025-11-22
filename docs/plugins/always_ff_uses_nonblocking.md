# always_ff_uses_nonblocking

- **Script**: `plugins/always_ff_uses_nonblocking.cst.py`
- **Stage**: `cst`
- **Key Inputs**: `cst_ir.tokens`, `tok_kind_table`, `line_starts`, `pp_text`
- **Summary**: Ban blocking `=` assignments inside `always_ff`

## Details

### Message
`` blocking '=' inside always_ff ``
### Good

```systemverilog
module m;
  logic a, clk;
  always_ff @(posedge clk) begin
    a <= 1'b1;
  end
endmodule
```

### Bad

```systemverilog
module m;
  logic a, clk;
  always_ff @(posedge clk) begin
    a = 1'b1;
  end
endmodule
```
