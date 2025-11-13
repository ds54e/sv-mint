module fork_label_violation;
  initial begin
    fork : iso_fork
      #1;
    join_any
    disable iso_fork;
  end
endmodule
