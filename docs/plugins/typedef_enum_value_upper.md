# typedef_enum_value_upper

- **Script**: `plugins/typedef_enum_value_upper.cst.py`
- **Stage**: `cst` (`mode = inline`)
- **Key Inputs**: `cst_ir.tokens`, `line_starts`, `pp_text`
- **Summary**: Enum members must be UpperCamelCase or ALL_CAPS

## Details

### Trigger
Enum members that are not `UpperCamelCase` or `ALL_CAPS`.
### Remediation
Capitalize each word (`UartInterruptFrameErr`) or use ALL_CAPS (`UART_MODE_IDLE`) to match the doc's readability requirement.
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
  uart_mode_busy
} uart_mode_e;
```
