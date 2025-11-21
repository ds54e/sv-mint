`default_nettype none

module always_plain(
  input  logic clk_i,
  input  logic rst_ni,
  input  logic a_i,
  output logic b_o
);

always @(posedge clk_i or negedge rst_ni) begin
  b_o <= !rst_ni ? 1'b0 : a_i;
end

endmodule

`default_nettype wire
