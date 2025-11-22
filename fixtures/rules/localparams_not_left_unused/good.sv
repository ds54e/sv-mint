module m #(
  localparam int MyParam1 = 1,
  localparam int MyParam2 = 1, // reserved
  localparam int MyParam3 = 1 // will be used later
)(
  input logic [MyParam1:0] a
);
endmodule
