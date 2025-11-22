`default_nettype none

module ports_not_left_unused_comment_good(
  input  logic debug_i,  // unused
  output logic ready_o
);
  assign ready_o = debug_i;
endmodule

`default_nettype wire
