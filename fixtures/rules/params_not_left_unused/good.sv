module m #(
  parameter int MyParam1 = 1,
  parameter int MyParam2 = 1, // reserved
  parameter int MyParam3 = 1 // will be used later
)(
  input logic [MyParam1:0] a
);
endmodule
