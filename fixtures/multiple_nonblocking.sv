module multiple_nonblocking (
  input logic clk_i,
  input logic rst_ni,
  input logic a_i,
  input logic b_i,
  output logic y_o
);

always_ff @(posedge clk_i or negedge rst_ni) begin
  if (!rst_ni) begin
    y_o <= 1'b0;
  end else begin
    y_o <= a_i;
  end
end

always_ff @(posedge clk_i) begin
  y_o <= b_i;
end

endmodule
