module logging_violation;
  initial begin
    uvm_info("TAG", "msg", UVM_FULL);
    uvm_warning("TAG", "warn");
    uvm_report_info("TAG", "info", UVM_MEDIUM);
    $display("legacy");
  end
endmodule
