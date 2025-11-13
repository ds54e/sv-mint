`define uvm_do(item) item

class uvm_do_violation;
  task body();
    `uvm_do(req)
  endtask
endclass

`undef uvm_do
