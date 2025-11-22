`default_nettype none

module var_lower_snake_ok;
  output logic good_var;
  output logic dollar_var;
  int good_local;
  assign good_var = 1'b0;
  assign dollar_var = good_var;
  initial good_local = 0;
endmodule

`default_nettype wire
