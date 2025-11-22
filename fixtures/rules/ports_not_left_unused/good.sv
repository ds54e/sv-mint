module m (
  input logic a,
  input logic b1, // reserved
  input logic b2, // used
  output logic c,
  output logic d1,
  output logic d2
);
  assign c = a;
  my_module inst (.d1, d2);
endmodule

