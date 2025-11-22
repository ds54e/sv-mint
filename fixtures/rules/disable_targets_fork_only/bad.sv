module m;
  initial begin
    fork : fork_label
      begin #1; end
      begin #2; end
    join_any
    disable fork_label;
  end
endmodule
