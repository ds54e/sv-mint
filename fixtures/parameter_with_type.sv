`default_nettype none

module parameter_with_type;
  parameter int WIDTH = 4;
  parameter signed [3:0] OFFSET = 0;
  parameter type T = int;
  localparam T VALUE = T'(WIDTH + OFFSET);
  logic [WIDTH-1:0] data;
  assign data = VALUE[WIDTH-1:0];
endmodule

`default_nettype wire
