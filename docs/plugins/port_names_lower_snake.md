# port_names_lower_snake

## Script
- `plugins/port_names_lower_snake.ast.py`

## Description
- Ports follow lower_snake_case + direction suffix

## Good

```systemverilog
module m (
  inout logic my_port_1,
  input logic my_port_2,
  output logic my_port_3
);
endmodule
```

## Bad

```systemverilog
module m1 (
  inout logic MyPort1,
  input logic MyPort2,
  output logic MyPort3
);
endmodule

module m2 (
  inout logic MY_PORT_1,
  input logic MY_PORT_2,
  output logic MY_PORT_3
);
endmodule
```
