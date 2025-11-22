# case_has_default_branch

## Script
- `plugins/case_has_default_branch.cst.py`

## Description
- Warn when a `case` statement lacks a `default` label (except for `unique`/`unique0 case`)
- Default branches prevent unintended latches and X-propagation when cases are unhandled.
## Good

```systemverilog
module m;

  logic [1:0] a;
  logic b, c;

  always_comb begin
    case (a)
      2'd0: b = 1'b0;
      2'd1: b = 1'b0;
      2'd2: b = 1'b0;
      default: b = 1'b0;
    endcase
  end

  always_comb begin
    unique case (a)
      2'd0: c = 1'b0;
      2'd1: c = 1'b0;
      2'd2: c = 1'b0;
      2'd3: c = 1'b0;
    endcase
  end

endmodule
```

## Bad

```systemverilog
module m;

  logic [1:0] a;
  logic b;

  always_comb begin
    case (a)
      2'd0: b = 1'b0;
      2'd1: b = 1'b0;
      2'd2: b = 1'b0;
    endcase
  end

endmodule
```
