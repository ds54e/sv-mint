`default_nettype none

module functions_have_explicit_types_good;
  function automatic logic [7:0] acc_fn(
    input logic [7:0] a_i,
    input logic [7:0] b_i
  );
    acc_fn = a_i + b_i;
  endfunction
endmodule

`default_nettype wire
