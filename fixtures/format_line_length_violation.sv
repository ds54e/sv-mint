module long_line_example (
  input logic clk_i,
  input logic rst_ni,
  input logic [31:0] data_bus_i,
  output logic ready_o
);

assign ready_o = (data_bus_i == 32'h0000_1234) ? 1'b1 : 1'b0;

assign ready_o = (data_bus_i == 32'hDEADBEEF) ? 1'b1 : 1'b0 && (data_bus_i != 32'hFACEFACE) && (data_bus_i != 32'hCAFEBABE) && (data_bus_i != 32'h0BADF00D);

endmodule
