`default_nettype none

module unused_port_violation(input logic unused_i, output logic y_o);
  assign y_o = 1'b0;
endmodule

`default_nettype wire
