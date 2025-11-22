`default_nettype none

module always_ff_uses_nonblocking_good(input logic clk_i, output logic a_o);
  always_ff @(posedge clk_i) begin
    a_o <= 1'b1;
  end
endmodule

`default_nettype wire
