`default_nettype none
`define USED_MACRO(x) (x + 1)

module macro_used;
  localparam int FOO = `USED_MACRO(1);
  output logic [7:0] bar_o;
  assign bar_o = FOO;
endmodule

`undef USED_MACRO
`default_nettype wire
