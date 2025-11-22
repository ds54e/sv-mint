// Example unique case without default; all values covered
`default_nettype none

module case_has_default_branch_good (
  input  logic [1:0] state_i,
  output logic done_o
);

always_comb begin
  unique case (state_i)
    2'd0: done_o = 1'b0;
    2'd1: done_o = 1'b1;
    2'd2: done_o = 1'b1;
    2'd3: done_o = 1'b0;
  endcase
end

endmodule

`default_nettype wire
