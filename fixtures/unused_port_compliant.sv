`default_nettype none

module unused_port_compliant(input logic a_i, output logic b_o);
  assign b_o = a_i;
endmodule

`default_nettype wire
