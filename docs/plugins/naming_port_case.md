# naming_port_case.py

- **Script**: `plugins/naming.port_case.ast.py`
- **Stage**: `ast`
- **Key Inputs**: `decls`, `symbols`, `ports`
- **Shared Helpers**: `plugins/lib/naming_ruleset.py`
- **Rule**:
  - ``naming.port_case`` (warning): Ports follow lower_snake_case + direction suffix

## Rule Details

### `naming.port_case`
#### Trigger
Verifies that port names follow lower_snake_case before suffixes are considered.
#### Message
`` port <name> must use lower_snake_case ``
#### Remediation
Rename ports to lowercase snake case and then apply direction suffix rules.
#### Good

```systemverilog
input  logic req_i;
output logic gnt_o;
```

#### Bad

```systemverilog
input logic Req;
output logic Grant;
```
