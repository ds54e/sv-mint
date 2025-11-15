# typedef_enum_value_prefix.py

- **Script**: `plugins/typedef.enum_value_prefix.raw.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Shared Helpers**: `plugins/lib/typedef_naming_ruleset.py`
- **Summary**: Enum members must start with the enum's CamelCase prefix

## Details

### Trigger
Enum members whose names do not start with the CamelCase form of the enum type (type name without `_e`).
### Remediation
Include the enum base in every member (`AonTimerModeDisabled`) to keep DV logs searchable, as recommended by DVCodingStyle.
### Good

```systemverilog
typedef enum logic [1:0] {
  AonTimerModeIdle,
  AonTimerModeRun
} aon_timer_mode_e;
```

### Bad

```systemverilog
typedef enum logic [1:0] {
  Idle,
  Run
} aon_timer_mode_e;
```
