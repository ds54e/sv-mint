module lang_violations (
  input logic clk_i
);

always @* begin
  #5 $display("delay");
end

always_latch begin
end

endmodule
