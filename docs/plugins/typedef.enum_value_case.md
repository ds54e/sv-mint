# typedef.enum_value_case

- **Script**: `plugins/typedef.naming.cst.py`
- **Stage**: `cst` (`mode = inline`)
- **Key Inputs**: `cst_ir.tokens`, `line_starts`, `pp_text`
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
