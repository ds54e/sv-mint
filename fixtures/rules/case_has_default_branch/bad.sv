module m;

  logic [1:0] a;
  logic b;

  always_comb begin
    case (a)
      2'd0: b = 1'b0;
      2'd1: b = 1'b0;
      2'd2: b = 1'b0;
    endcase
  end

endmodule
