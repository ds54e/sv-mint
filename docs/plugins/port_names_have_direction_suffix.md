# port_names_have_direction_suffix

## Script
- `plugins/port_names_have_direction_suffix.ast.py`

## Description
- `_i/_o/_io` suffixes must match port direction
- Why: Direction suffixes encode intent for readers and reduce hookup errors.
## Good

```systemverilog
module m (
  inout logic a_io
  input logic b_i,
  output logic c_o
);
endmodule
```

## Bad

```systemverilog
module m (
  inout logic a,
  input logic b,
  output logic c
);
endmodule
```
