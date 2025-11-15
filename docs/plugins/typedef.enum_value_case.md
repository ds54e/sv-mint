# typedef.enum_value_case

- **Script**: `plugins/typedef.enum_value_case.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Shared Helpers**: `plugins/lib/typedef_naming_ruleset.py`
- **Summary**: Enum members must be UpperCamelCase

## Details

### Trigger
Enum members that are not `UpperCamelCase`.
### Remediation
Capitalize each word (`UartInterruptFrameErr`) to match the doc's readability requirement.
### Good

```systemverilog
typedef enum logic [1:0] {
  UartModeIdle,
  UartModeBusy
} uart_mode_e;
```

### Bad

```systemverilog
typedef enum logic [1:0] {
  UART_MODE_IDLE,
  uart_mode_busy
} uart_mode_e;
```
