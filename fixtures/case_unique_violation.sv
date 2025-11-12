module case_unique_violation (
  input logic [1:0] state_i,
  output logic flag_o
);

always_comb begin
  case (state_i)
    2'd0: flag_o = 1'b0;
    2'd1: flag_o = 1'b1;
    default: flag_o = 1'b0;
  endcase
end

endmodule
