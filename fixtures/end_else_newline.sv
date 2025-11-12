module end_else_newline (
  input  logic sel_i,
  input  logic a_i,
  input  logic b_i,
  output logic y_o
);

always_comb begin
  if (sel_i) begin
    y_o = a_i;
  end
  else begin
    y_o = b_i;
  end
end

endmodule
