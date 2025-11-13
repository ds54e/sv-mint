package logging_pkg;
  function void uvm_info(string tag, string msg, int verbosity);
  endfunction
  function void uvm_warning(string tag, string msg);
  endfunction
  function void uvm_report_info(string tag, string msg, int verbosity);
  endfunction
  function void uvm_error(string tag, string msg, int verbosity);
  endfunction
endpackage

module logging_violation;
  import logging_pkg::*;
  localparam int UVM_LOW = 1;
  localparam int UVM_MEDIUM = 2;
  localparam int UVM_HIGH = 3;
  localparam int UVM_DEBUG = 4;
  localparam int UVM_FULL = 5;
  localparam string gfn = "dut";
  initial begin
    uvm_info("TAG", "msg", UVM_FULL);
    uvm_warning("TAG", "warn");
    uvm_report_info("TAG", "info", UVM_MEDIUM);
    $display("legacy");
    uvm_error(gfn, "detail", 1);
  end
endmodule
