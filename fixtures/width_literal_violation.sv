module width_literal_violation;
  logic [7:0] value;
  initial begin
    value = 'hFF;
  end
endmodule
