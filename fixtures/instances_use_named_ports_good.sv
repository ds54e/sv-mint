`default_nettype none

module instances_use_named_ports_good;
  logic clk;
  logic done;
  child u_child(
    .clk_i(clk),
    .done_o(done)
  );
endmodule

`default_nettype wire
