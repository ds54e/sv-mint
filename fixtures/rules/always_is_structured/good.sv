module m;

  logic a, b;
  logic clk;

  always_ff @(posedge clk) begin
    a <= 1'b1;
  end

  always_latch begin
    if (clk) begin
      b <= 1'b1;
    end
  end

  logic c;

  always_comb begin
    c = 1'b1;
  end

endmodule
