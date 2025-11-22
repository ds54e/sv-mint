# case_has_default_branch

- **Script**: `plugins/case_has_default_branch.cst.py`
- **Stage**: `cst`
- **Key Inputs**: `cst_ir.tokens`, `line_starts`, `pp_text`
- **Summary**: Warn when a `case` statement lacks a `default` label (except for `unique`/`unique0 case`)

## Details

### Message
`` case statement must include a default item ``
### Remediation
Add a `default` branch when using plain `case`. `unique`/`unique0` cases are exempt (the rule does not check exhaustiveness).
### Good

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
```systemverilog

```systemverilog
case (state_q)  // default required when not unique
  IDLE:   state_d = START;
  START:  state_d = DONE;
  default: state_d = IDLE;  // handle unexpected states
endcase
```

### Bad

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
```systemverilog