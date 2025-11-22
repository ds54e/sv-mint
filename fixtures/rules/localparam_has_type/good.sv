`default_nettype none

module localparam_has_type_good;
  localparam int unsigned DEPTH = 16;
  localparam logic signed [3:0] OFFSET = -1;
  logic [DEPTH-1:0] data;
  assign data = {DEPTH{1'b0}} + OFFSET;
endmodule

`default_nettype wire
