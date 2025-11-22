# enum_values_uppercase

- **Script**: `plugins/enum_values_uppercase.cst.py`
- **Stage**: `cst`
- **Key Inputs**: `cst_ir.tokens`, `line_starts`, `pp_text`
- **Summary**: Enum members must be UpperCamelCase or ALL_CAPS

## Details

### Remediation
Capitalize each word (`UartInterruptFrameErr`) or use ALL_CAPS (`UART_MODE_IDLE`) to match the doc's readability requirement.
### Good

```systemverilog
typedef enum int unsigned {
  OFF,
  ON
} state_1_e;

typedef enum int unsigned {
  Off,
  On
} state_2_e;
```systemverilog

### Bad

```systemverilog
typedef enum int unsigned {
  off,
  on
} state_e;
```systemverilog