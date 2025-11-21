`default_nettype none

module unused_port_unused_comment(
  input  logic debug_i,  // unused
  output logic ready_o
);
  assign ready_o = 1'b0;
endmodule

`default_nettype wire
