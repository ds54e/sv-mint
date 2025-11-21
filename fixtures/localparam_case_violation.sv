// SPDX-License-Identifier: Apache-2.0
`default_nettype none

module localparam_case_violation;
  localparam int bad_param = 1;
endmodule

`default_nettype wire
