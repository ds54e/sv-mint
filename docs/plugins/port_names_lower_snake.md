# port_names_lower_snake

- **Script**: `plugins/port_names_lower_snake.ast.py`
- **Stage**: `ast`
- **Key Inputs**: `decls`, `symbols`, `ports`
- **Summary**: Ports follow lower_snake_case + direction suffix

## Details

### Message
`` port <name> must use lower_snake_case ``
### Remediation
Rename ports to lowercase snake case and then apply direction suffix rules.
### Good

```systemverilog
module m (
  inout logic my_port_1,
  input logic my_port_2,
  output logic my_port_3
);
endmodule
```systemverilog

### Bad

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
```systemverilog