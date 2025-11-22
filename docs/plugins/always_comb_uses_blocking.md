# always_comb_uses_blocking

## Script
- `plugins/always_comb_uses_blocking.cst.py`

## Description
- Prohibit non-blocking assignments (`<=`) inside `always_comb`
- Why: Non-blocking in combinational logic can create delta-cycle races and unintended latches; blocking reflects true combinational intent.
## Good

```systemverilog
module m;
  logic a;
  always_comb begin
    a = 1'b1;
  end
endmodule
```

## Bad

```systemverilog
module m;
  logic a;
  always_comb begin
    a <= 1'b1;
  end
endmodule
```
