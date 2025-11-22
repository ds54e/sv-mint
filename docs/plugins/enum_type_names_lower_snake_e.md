# enum_type_names_lower_snake_e

## Script
- `plugins/enum_type_names_lower_snake_e.cst.py`

## Description
- Enum type names must be lower_snake_case and end with `_e`
- Consistent enum type naming distinguishes types from variables at a glance.
## Good

```systemverilog
typedef enum int unsigned {
  OFF,
  ON
} state_e;
```

## Bad

```systemverilog
typedef enum int unsigned {
  OFF,
  ON
} state;
```
