module m;
  logic a, clk;
  always_ff @(posedge clk) begin
    a <= 1'b1;
  end
endmodule
