`default_nettype none

`define GOOD_MACRO 1

module macro_names_uppercase_good(output logic val_o);
  assign val_o = `GOOD_MACRO;
endmodule

`default_nettype wire

`undef GOOD_MACRO
