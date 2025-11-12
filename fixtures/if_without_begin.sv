module if_without_begin (
  input  logic sel_i,
  input  logic a_i,
  input  logic b_i,
  output logic y_o
);

always_comb begin
  if (sel_i)
    y_o = a_i;
  else
    y_o = b_i;
end

endmodule
