`default_nettype none

module localparam_names_uppercase_good(output wire [WIDTH-1:0] data_o);
  localparam int GOOD_PARAM = 1;
  localparam int CAMEL_CASE_PARAM = 2;
  localparam int WIDTH = GOOD_PARAM + CAMEL_CASE_PARAM;
  assign data_o = {WIDTH{1'b0}};
endmodule

`default_nettype wire
