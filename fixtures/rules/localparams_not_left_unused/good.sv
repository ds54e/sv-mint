`default_nettype none

module localparams_not_left_unused_good;
  localparam int EnableDbg = 0; // reserved
  localparam int Depth = 4;
  logic [Depth-1:0] data;
  assign data = {Depth{1'b0}} + EnableDbg;
endmodule

`default_nettype wire
