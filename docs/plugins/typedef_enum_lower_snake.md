# typedef_enum_lower_snake.py

- **Script**: `plugins/typedef.enum_lower_snake.raw.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Shared Helpers**: `plugins/lib/typedef_naming_ruleset.py`
- **Rule**:
  - ``typedef.enum_lower_snake`` (warning): Enum type names must use `lower_snake_case`

## Rule Details

### `typedef.enum_lower_snake`
#### Trigger
Enum type names not written in `lower_snake_case`.
#### Remediation
Follow the DVCodingStyle guidance and adopt names such as `uart_interrupt_e` or `aon_timer_mode_e` so the enum base is unambiguous.
#### Good

```systemverilog
typedef enum logic [1:0] {
  IDLE,
  BUSY
} uart_mode_e;
```

#### Bad

```systemverilog
typedef enum logic [1:0] {
  IDLE,
  BUSY
} UartMode_e;
```
