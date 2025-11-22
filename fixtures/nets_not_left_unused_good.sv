// Decl unused net compliant
`default_nettype none

module nets_not_left_unused_good;

wire debug_tap; // unused

endmodule

`default_nettype wire
