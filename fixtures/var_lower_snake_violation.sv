// SPDX-License-Identifier: Apache-2.0
`default_nettype none

module var_lower_snake_violation;
  int BadVar;
  initial BadVar = 0;
endmodule

`default_nettype wire
