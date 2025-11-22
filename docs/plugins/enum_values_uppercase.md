# enum_values_uppercase

## Script
- `plugins/enum_values_uppercase.cst.py`

## Description
- Enum members must be UpperCamelCase or ALL_CAPS
- Why: ALL_CAPS enum values read as constants and reduce confusion with signals.
## Good

```systemverilog
typedef enum int unsigned {
  OFF,
  ON
} state_1_e;

typedef enum int unsigned {
  Off,
  On
} state_2_e;
```

## Bad

```systemverilog
typedef enum int unsigned {
  off,
  on
} state_e;
```
