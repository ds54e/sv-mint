// Decl unused localparam violation
`default_nettype none

module unused_localparam_violation;
  localparam int EnableDbg = 0;
endmodule

`default_nettype wire
