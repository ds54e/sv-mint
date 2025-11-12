module always_ff_violation (
  input logic clk_i,
  input logic rst_ni,
  input logic a_i,
  output logic y_o
);
  always_ff @(posedge clk_i) begin
    y_o <= a_i;
  end
  always_comb @(posedge clk_i) begin
    y_o = a_i;
  end
endmodule
