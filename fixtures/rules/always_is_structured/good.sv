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
