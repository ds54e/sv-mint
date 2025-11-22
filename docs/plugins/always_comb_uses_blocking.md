# always_comb_uses_blocking

## Script
- `plugins/always_comb_uses_blocking.cst.py`

## Description

- Prohibit non-blocking assignments (`<=`) inside `always_comb`

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
