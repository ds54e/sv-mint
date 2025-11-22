`default_nettype none

module macros_use_module_prefix_good(output logic val_o);
`define MACROS_USE_MODULE_PREFIX_GOOD_FOO 1
  assign val_o = `MACROS_USE_MODULE_PREFIX_GOOD_FOO;
`undef MACROS_USE_MODULE_PREFIX_GOOD_FOO
endmodule

`default_nettype wire
