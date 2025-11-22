`default_nettype none

module functions_marked_automatic_or_static_good;
  function automatic int add(input int a, input int b);
    add = a + b;
  endfunction
endmodule

`default_nettype wire
