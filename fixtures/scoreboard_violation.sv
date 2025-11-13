class foo_scoreboard extends uvm_component;
  function new(string name, uvm_component parent);
    super.new(name, parent);
  endfunction

  virtual function void check_phase(uvm_phase phase);
    // missing DV_EOT_PRINT macros
  endfunction
endclass
