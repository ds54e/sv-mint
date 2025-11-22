`default_nettype none

module port_names_lower_snake_good(
  input logic clk_i,
  output logic data_o
);
  assign data_o = clk_i;
endmodule

`default_nettype wire
