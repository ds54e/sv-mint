`define GLOBAL_CONST 4
`define _LOCAL_MACRO(x) assign foo = x;

module global_define_violations (
  input logic foo
);

endmodule
