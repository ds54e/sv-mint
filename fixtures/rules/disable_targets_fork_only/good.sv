module m;

  initial begin
    fork
      begin #1; end
      begin #2; end
    join_any
    disable fork;
  end

endmodule
