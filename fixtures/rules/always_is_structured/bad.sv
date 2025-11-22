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
