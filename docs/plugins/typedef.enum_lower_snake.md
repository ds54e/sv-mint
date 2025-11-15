# typedef.enum_lower_snake

- **Script**: `plugins/typedef.enum_lower_snake.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Shared Helpers**: `plugins/lib/typedef_naming_ruleset.py`
- **Summary**: Enum type names must use `lower_snake_case`

## Details

### Trigger
Enum type names not written in `lower_snake_case`.
### Remediation
Follow the DVCodingStyle guidance and adopt names such as `uart_interrupt_e` or `aon_timer_mode_e` so the enum base is unambiguous.
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
