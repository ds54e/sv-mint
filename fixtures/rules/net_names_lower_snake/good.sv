`default_nettype none

module net_names_lower_snake_good(
  output logic good_name_o,
  output logic dollar_name_o
);
  assign good_name_o = 1'b0;
  assign dollar_name_o = good_name_o;
endmodule

`default_nettype wire
