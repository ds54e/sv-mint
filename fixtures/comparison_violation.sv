module comparison_violation;
  logic a, b;
  initial begin
    if (a != b) begin
      uvm_error("cmp", "mismatch", UVM_LOW);
    end
  end
endmodule
