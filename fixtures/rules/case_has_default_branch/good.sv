module m;

  logic [1:0] a;
  logic b, c;

  always_comb begin
    case (a)
      2'd0: b = 1'b0;
      2'd1: b = 1'b0;
      2'd2: b = 1'b0;
      default: b = 1'b0;
    endcase
  end

  always_comb begin
    unique case (a)
      2'd0: c = 1'b0;
      2'd1: c = 1'b0;
      2'd2: c = 1'b0;
      2'd3: c = 1'b0;
    endcase
  end

endmodule
