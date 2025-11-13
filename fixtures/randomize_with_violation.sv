class randomize_with_violation;
  function void run();
    req.randomize() with { addr inside {[0:3]}; };
  endfunction
endclass
