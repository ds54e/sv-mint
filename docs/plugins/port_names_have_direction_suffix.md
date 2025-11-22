# port_names_have_direction_suffix

- **Script**: `plugins/port_names_have_direction_suffix.ast.py`
- **Stage**: `ast`
- **Key Inputs**: `decls`, `symbols`, `ports`
- **Summary**: `_i/_o/_io` suffixes must match port direction

## Details

### Message
`` port <name> must use suffix matching its direction ``
### Remediation
Append `_i`, `_o`, or `_io` (with `_n` for active-low signals) so direction is obvious at call sites.
### Good

```systemverilog
input  logic req_i;
input  logic rst_ni;
output logic data_o;
```

### Bad

```systemverilog
input  logic req;
output logic data_out;
```
