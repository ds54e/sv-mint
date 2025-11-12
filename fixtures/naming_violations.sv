module BadModuleName (
  input logic dramClk_i,
  input logic rst_i,
  input logic clk_aux_i,
  output logic DATA_OUT_O
);

  logic DataSig;

  always_comb begin
    DataSig = dramClk_i;
  end

endmodule
