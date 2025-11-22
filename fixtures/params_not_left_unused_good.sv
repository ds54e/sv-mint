// Decl unused param compliant
`default_nettype none

module params_not_left_unused_good #(
  parameter int EnableDbg = 0  // unused
) ();

endmodule

`default_nettype wire
