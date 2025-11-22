module m;

  `define MACRO_A(a) a
  `define MACRO_B(b) b // reserved
  `define MACRO_C(c) \
    if (c) begin \
      $display(1); \
    end else else \
      $display(0); \
    end // will be used later

  wire y = `MACRO_A(1'b1);

endmodule
