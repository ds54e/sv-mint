# vars_not_left_unused

- **Script**: `plugins/vars_not_left_unused.ast.py`
- **Stage**: `ast`
- **Key Inputs**: `symbols` entries with `class == var`
- **Summary**: Warn about variable declarations that are never referenced

## Details

### Notes
- Implicit `.*` connections are not elaborated; they will be counted as unused.
### Good

```systemverilog
module m;

  logic a;
  always_comb a = 1'b0;

  logic b = 1'bz;

  logic c1; // reserved
  logic c2; // used

  logic d;
  function fn (in); return 1'b0; endfunction
  logic e = fn(d);

  logic f;
  wire g = (f ? 1'b1 : 1'b0);

  logic h;
  always_comb begin
    if (h) begin
      $display(1);
    end else begin
      $display(0);
    end
  end

  logic i;
  initial begin
    $display(i);
  end

  logic j;
  logic k;
  my_module inst (.j, .k);

endmodule
```

### Bad

```systemverilog
module m;
  logic a;
endmodule
```
