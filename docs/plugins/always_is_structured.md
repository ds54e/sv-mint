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

### Bad

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
