`default_nettype none

module function_missing_types;
  function acc_fn(input a_i, input b_i);
    acc_fn = a_i + b_i;
  endfunction
endmodule

`default_nettype wire
