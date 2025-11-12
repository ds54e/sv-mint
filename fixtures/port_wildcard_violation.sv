module child (
  input logic clk_i,
  input logic rst_ni,
  input logic req_i,
  output logic ack_o
);
  assign ack_o = req_i;
endmodule

module parent (
  input logic clk_i,
  input logic rst_ni,
  input logic req_i,
  output logic ack_o
);

  child u_child (
    .*
  );

endmodule
