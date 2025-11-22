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

endmodule

