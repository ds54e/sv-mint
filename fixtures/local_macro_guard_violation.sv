`ifndef LOCAL_HELPER
`define LOCAL_HELPER(x) (x)
`endif

module local_macro_guard_violation;
  initial begin
    LOCAL_HELPER(1);
  end
endmodule

`undef LOCAL_HELPER
