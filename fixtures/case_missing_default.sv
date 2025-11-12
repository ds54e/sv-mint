module case_missing_default (
  input  logic clk_i,
  input  logic rst_ni,
  input  logic [1:0] state_i,
  output logic done_o
);

always_ff @(posedge clk_i or negedge rst_ni) begin
  if (!rst_ni) begin
    done_o <= 1'b0;
  end else begin
    unique case (state_i)
      2'd0: done_o <= 1'b0;
      2'd1: done_o <= 1'b1;
    endcase
  end
end

endmodule
