# always_ff_uses_nonblocking

## Script
- `plugins/always_ff_uses_nonblocking.cst.py`

## Description
- Ban blocking `=` assignments inside `always_ff`

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
