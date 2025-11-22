`default_nettype none

module localparam_missing_type;
  localparam DEPTH = 16;
  logic [DEPTH-1:0] payload;
  assign payload = {DEPTH{1'b0}};
endmodule

`default_nettype wire
