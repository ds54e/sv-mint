# always_ff_uses_nonblocking

## Script
- `plugins/always_ff_uses_nonblocking.cst.py`

## Description
- Ban blocking `=` assignments inside `always_ff`
- Non-blocking assignments preserve sequential semantics and avoid read-before-write races in flip-flop logic.
## Good

```systemverilog
module m;
  logic a, clk;
  always_ff @(posedge clk) begin
    a <= 1'b1;
  end
endmodule
```

## Bad

```systemverilog
module m;
  logic a, clk;
  always_ff @(posedge clk) begin
    a = 1'b1;
  end
endmodule
```
