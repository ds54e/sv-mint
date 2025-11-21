# typedef.type_name_lower_snake_t

- **Script**: `plugins/typedef.type_name_lower_snake_t.cst.py`
- **Stage**: `cst` (`mode = inline`)
- **Key Inputs**: `cst_ir.tokens`, `line_starts`, `pp_text`
- **Summary**: Typedef names (non-enum) must be lower_snake_case and end with `_t`

## Details

### Trigger
Flags `typedef` names (excluding enums) that are not `lower_snake_case` or do not end with `_t`.

### Message
`` typedef names should use lower_snake_case and end with _t: <name> ``

### Remediation
Rename typedefs such as `data_width_t`, `foo_bar_t`, etc.

### Good

```systemverilog
typedef logic [3:0] data_width_t;
typedef struct packed {
  logic a;
  logic b;
} packet_t;
```

### Bad

```systemverilog
typedef logic [3:0] DataWidth;
typedef struct packed {
  logic a;
  logic b;
} Packet;
```
