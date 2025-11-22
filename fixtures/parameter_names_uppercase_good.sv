`default_nettype none

module parameter_names_uppercase_good(output logic [SUM_WIDTH-1:0] bus_o);
  parameter int DATA_WIDTH = 16;
  parameter int WIDTH = DATA_WIDTH;
  localparam int SUM_WIDTH = WIDTH + 1;
  assign bus_o = {SUM_WIDTH{1'b0}};
endmodule

`default_nettype wire
