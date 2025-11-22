`default_nettype none

module always_ff_blocking_violation(input logic clk);
  logic a;
  always_ff @(posedge clk) begin
    a = 1'b0;
  end
endmodule

`default_nettype wire
