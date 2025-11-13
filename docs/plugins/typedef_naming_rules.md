# typedef_naming_rules.py

- **Script**: `plugins/typedef_naming_rules.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Rules**:
  | Rule ID | Severity | Summary |
  | --- | --- | --- |
  | `typedef.enum_suffix` | warning | Require `typedef enum` names to end with `_e` |
  | `typedef.enum_lower_snake` | warning | Enum type names must use `lower_snake_case` |
  | `typedef.enum_value_case` | warning | Enum members must be UpperCamelCase |
  | `typedef.enum_value_prefix` | warning | Enum members must start with the enum's CamelCase prefix |
  | `typedef.type_suffix` | warning | Require other typedef names to end with `_t` |

## Rule Details

### `typedef.enum_suffix`
- **Trigger**: Matches `typedef enum { ... } name;` constructs whose `name` does not end with `_e`.
- **Message**: `` enum types should end with _e: state ``
- **Remediation**: Rename to `state_e`, etc.
- **LowRISC Reference**: Enumerated types use `_e`, while enum values stay in UpperCamelCase.
- **Additional Tips**: Do not use `_t` for enums; that conflicts with the struct rule below.

### `typedef.enum_lower_snake`
- **Trigger**: Enum type names not written in `lower_snake_case`.
- **Remediation**: Follow the DVCodingStyle guidance and adopt names such as `uart_interrupt_e` or `aon_timer_mode_e` so the enum base is unambiguous.

### `typedef.enum_value_case`
- **Trigger**: Enum members that are not `UpperCamelCase`.
- **Remediation**: Capitalize each word (`UartInterruptFrameErr`) to match the doc's readability requirement.

### `typedef.enum_value_prefix`
- **Trigger**: Enum members whose names do not start with the CamelCase form of the enum type (type name without `_e`).
- **Remediation**: Include the enum base in every member (`AonTimerModeDisabled`) to keep DV logs searchable, as recommended by DVCodingStyle.

### `typedef.type_suffix`
- **Trigger**: Non-enum typedef names lacking `_t`.
- **Message**: `` typedef names should end with _t: data ``
- **Remediation**: Append `_t`, e.g., `data_t`.
- **LowRISC Reference**: Structs, packed arrays, and other types end in `_t`.
- **Additional Tips**: When exporting typedefs from packages, keep the `_t` suffix for downstream consistency.
- **Good**:

```systemverilog
typedef enum logic [1:0] {
  IDLE,
  BUSY
} state_e;

typedef struct packed {
  logic valid;
  logic [31:0] data;
} payload_t;
```

- **Bad**:

```systemverilog
typedef enum logic [1:0] {
  IDLE,
  BUSY
} state;

typedef struct packed {
  logic valid;
  logic [31:0] data;
} payload;
```
