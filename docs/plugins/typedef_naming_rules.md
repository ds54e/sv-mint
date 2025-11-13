# typedef_naming_rules.py

- **Script**: `plugins/typedef_naming_rules.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Rules**:
  | Rule ID | Severity | Summary |
  | --- | --- | --- |
  | `typedef.enum_suffix` | warning | Require `typedef enum` names to end with `_e` |
  | `typedef.type_suffix` | warning | Require other typedef names to end with `_t` |

## Rule Details

### `typedef.enum_suffix`
- **Trigger**: Matches `typedef enum { ... } name;` constructs whose `name` does not end with `_e`.
- **Message**: `` enum types should end with _e: state ``
- **Remediation**: Rename to `state_e`, etc.
- **LowRISC Reference**: Enumerated types use `_e`, while enum values stay in UpperCamelCase.
- **Additional Tips**: Do not use `_t` for enums; that conflicts with the struct rule below.

### `typedef.type_suffix`
- **Trigger**: Non-enum typedef names lacking `_t`.
- **Message**: `` typedef names should end with _t: data ``
- **Remediation**: Append `_t`, e.g., `data_t`.
- **LowRISC Reference**: Structs, packed arrays, and other types end in `_t`.
- **Additional Tips**: When exporting typedefs from packages, keep the `_t` suffix for downstream consistency.
