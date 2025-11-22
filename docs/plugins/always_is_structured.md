# always_is_structured

## Script
- `plugins/always_is_structured.cst.py`

## Description
- Replace bare `always` with `always_ff`/`always_comb`/`always_latch`

## Good

```systemverilog
module m;

  logic a, b, c;
  logic clk;

  always_ff @(posedge clk) begin
    a <= 1'b1;
  end

  always_latch begin
    if (clk) begin
      b <= 1'b1;
    end
  end

  always_comb begin
    c = 1'b1;
  end

endmodule
```

```systemverilog
always_comb begin
  state_d = next_state;
end
```

## Bad

```
module m;

  logic a, b, c;
  logic clk;

  always @(posedge clk) begin
    a <= 1'b1;
  end

  always @* begin
    if (clk) begin
      b <= 1'b1;
    end
  end

  always @* begin
    c = 1'b1;
  end

endmodule
```systemverilog

```
always @* begin
  state_d = next_state;
end
```
