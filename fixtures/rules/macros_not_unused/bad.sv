module m;

  `define MACRO_A(a) a
  `define MACRO_B(b) b
  `define MACRO_C(c) \
    if (c) begin \
      $display(1); \
    end else else \
      $display(0); \
    end

endmodule

