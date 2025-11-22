# enum_type_names_lower_snake_e

- **Script**: `plugins/enum_type_names_lower_snake_e.cst.py`
- **Stage**: `cst`
- **Key Inputs**: `cst_ir.tokens`, `line_starts`, `pp_text`
- **Summary**: Enum type names must be lower_snake_case and end with `_e`

## Details

### Good

```systemverilog
typedef enum int unsigned {
  OFF,
  ON
} state_e;
```

### Bad

```systemverilog
typedef enum int unsigned {
  OFF,
  ON
} state;
```
