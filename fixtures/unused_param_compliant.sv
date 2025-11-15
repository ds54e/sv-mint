// SPDX-License-Identifier: Apache-2.0
// Decl unused param compliant
`default_nettype none

module unused_param_compliant #(
  parameter int EnableDbg = 0  // unused
) ();

endmodule

`default_nettype wire
