module wait_violation;
  bit ready;
  initial begin
    wait (ready);
    fork
      ready = 1'b0;
    join_any
    wait fork;
  end
endmodule
