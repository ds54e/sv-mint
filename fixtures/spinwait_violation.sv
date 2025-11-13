module spinwait_violation;
  bit done;
  task automatic poll();
    while (!done) begin
      #1;
    end
  endtask
endmodule
