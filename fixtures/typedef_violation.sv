// SPDX-License-Identifier: Apache-2.0
`default_nettype none

typedef enum logic [1:0] { A, B } BadEnum;
typedef logic [3:0] BadType;

module typedef_violation;
endmodule

`default_nettype wire
