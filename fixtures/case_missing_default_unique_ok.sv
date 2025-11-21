module case_missing_default_unique_ok (
  input  logic clk_i,
  input  logic [1:0] state_i,
  output logic done_o
);

always_ff @(posedge clk_i) begin
  unique case (state_i)
    2'd0: done_o <= 1'b0;
    2'd1: done_o <= 1'b1;
    2'd2: done_o <= 1'b1;
    2'd3: done_o <= 1'b0;
  endcase
end

endmodule
