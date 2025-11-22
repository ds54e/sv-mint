`default_nettype none

module child(input logic a);
endmodule

module parent(input logic a);
  child u_child (a);
  child u_child2 ( .a(a) );
  child u_child3 (.*);
endmodule

`default_nettype wire
