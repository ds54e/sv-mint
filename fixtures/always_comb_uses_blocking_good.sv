`default_nettype none

module always_comb_uses_blocking_good(input logic b_i, output logic a_o);
  always_comb begin
    a_o = b_i;
  end
  initial begin
    $display(\"%b\", a_o);
  end
endmodule

`default_nettype wire
