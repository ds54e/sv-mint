// Decl unused var compliant
`default_nettype none

module vars_not_left_unused_good;

logic debug_shadow; // unused

endmodule

`default_nettype wire
