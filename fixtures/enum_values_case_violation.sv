`default_nettype none

typedef enum logic [1:0] {
  bad_value
} enum_values_case_violation_e;

module enum_values_case_violation;
  enum_values_case_violation_e state;
endmodule

`default_nettype wire
