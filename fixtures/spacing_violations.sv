module spacing_violations;

  function automatic logic passthru(input logic value);
    return value;
  endfunction

  logic a, b;

  initial begin
    passthru (a);
    unique case (a)
      1'b0 :a = 1'b0;
      default:b = 1'b1;
    endcase
  end

  some_module u_some (
    .a(a),.b(b)
  );

endmodule
