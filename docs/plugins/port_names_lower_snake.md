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
input  logic req_i;
output logic gnt_o;
```

### Bad

```systemverilog
input logic Req;
output logic Grant;
```
