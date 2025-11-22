`default_nettype none

module ports_not_left_unused_good(input logic a_i, output logic b_o);
  assign b_o = a_i;
endmodule

`default_nettype wire
