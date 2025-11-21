// SPDX-License-Identifier: Apache-2.0
`default_nettype none

module localparam_case_ok;
  localparam int GOOD_PARAM = 1;
  localparam int CamelCaseParam = 2;
endmodule

`default_nettype wire
