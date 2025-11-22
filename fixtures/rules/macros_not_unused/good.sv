module m;

  `define MACRO_A(a) a
  `define MACRO_B(b) b
  `define MACRO_C(c) \
    if (c) begin \
      $display(1); \
    end else else \
      $display(0); \
    end

  wire y = `MACRO_A(1'b1);
  wire z = `MACRO_B(1'b0);

  initial begin
    `MACRO_C(1'b1)
  end

endmodule
