`default_nettype none

`define GOOD_MACRO 1

module macros_close_with_undef_good(output logic val_o);
  assign val_o = `GOOD_MACRO;
endmodule

`undef GOOD_MACRO

`default_nettype wire
