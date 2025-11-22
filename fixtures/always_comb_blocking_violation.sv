`default_nettype none

module always_comb_blocking_violation;
  logic a;
  always_comb begin
    a <= 1'b0;
  end
endmodule

`default_nettype wire
