# nets_not_left_unused

## Script
- `plugins/nets_not_left_unused.ast.py`

## Description
- Warn when declared nets are never read or written.
- Implicit `.*` connections are not elaborated; they will be counted as unused.
## Good

```systemverilog
module m;

  wire a;
  assign a = 1'bz;

  wire b = 1'bz;

  wire c1; // reserved
  wire c2; // used

  wire d;
  function fn (in); return 1'b0; endfunction
  wire e = fn(d);

  wire f;
  wire g = (f ? 1'b1 : 1'b0);

  wire h;
  always_comb begin
    if (h) begin
      $display(1);
    end else begin
      $display(0);
    end
  end

  wire i;
  initial begin
    $display(i);
  end

  wire j;
  wire k;
  my_module inst (.j, .k);


endmodule
```

## Bad

```systemverilog
module m;
  wire a;
endmodule
```
