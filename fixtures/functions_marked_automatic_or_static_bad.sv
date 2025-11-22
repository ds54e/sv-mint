`default_nettype none

package function_scope_pkg;
function void helper();
endfunction
endpackage

module functions_marked_automatic_or_static_bad;
  function int add(input int a, input int b);
    add = a + b;
  endfunction
endmodule

`default_nettype wire
