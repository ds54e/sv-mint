# localparams_not_left_unused

## Script
- `plugins/localparams_not_left_unused.ast.py`

## Description
- Detect localparams whose reference count stays at zero
- Unused localparams imply stale configuration knobs or dead code.
## Good

```systemverilog
module m #(
  localparam int MyParam1 = 1,
  localparam int MyParam2 = 1, // reserved
  localparam int MyParam3 = 1 // will be used later
)(
  input logic [MyParam1:0] a
);
endmodule
```

## Bad

```systemverilog
module m #(
  localparam int MyParam = 1
);
endmodule
```
