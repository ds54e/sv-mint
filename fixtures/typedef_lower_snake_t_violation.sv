`default_nettype none

typedef logic [3:0] BadType;

module typedef_lower_snake_t_violation;
  BadType data;
endmodule

`default_nettype wire
