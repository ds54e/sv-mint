// Decl unused var compliant
`default_nettype none

module vars_not_left_unused_good;

logic debug_shadow; // unused
initial debug_shadow = 1'b0;

endmodule

`default_nettype wire
