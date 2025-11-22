`default_nettype none

module disable_fork_label_violation;
  initial begin
    fork : fork_label
      #1;
    join_any
    disable fork_label;
  end
endmodule

`default_nettype wire
