# typedef_enum_name_lower_snake_e

- **Script**: `plugins/typedef_enum_name_lower_snake_e.cst.py`
- **Stage**: `cst` (`mode = inline`)
- **Key Inputs**: `cst_ir.tokens`, `line_starts`, `pp_text`
- **Summary**: Enum type names must be lower_snake_case and end with `_e`

## Details

### Trigger
Flags `typedef enum` names that are not `lower_snake_case` or do not end with `_e`.

### Message
`` enum types should use lower_snake_case and end with _e: <name> ``

### Remediation
Rename enum types such as `uart_mode_e`, `aon_timer_state_e`, etc.

### Good

```systemverilog
typedef enum logic [1:0] {
  IDLE,
  BUSY
} uart_mode_e;
```

### Bad

```systemverilog
typedef enum logic [1:0] {
  IDLE,
  BUSY
} UartMode_e;
```
