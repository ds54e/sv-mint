`default_nettype none

module sensitivity_comma_ok;
  logic clk_i;
  logic rst_ni;
  logic data_d;
  logic data_q;

  always_ff @(posedge clk_i, negedge rst_ni) begin
    if (!rst_ni) data_q <= '0;
    else data_q <= data_d;
  end
endmodule

`default_nettype wire
