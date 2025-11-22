`default_nettype none

module function_with_explicit_types;
  function automatic logic [7:0] acc_fn(
    input logic [7:0] a_i,
    input logic [7:0] b_i
  );
    acc_fn = a_i + b_i;
  endfunction
endmodule

`default_nettype wire
