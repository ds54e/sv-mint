# enum_values_uppercase

- **Script**: `plugins/enum_values_uppercase.cst.py`
- **Stage**: `cst`
- **Key Inputs**: `cst_ir.tokens`, `line_starts`, `pp_text`
- **Summary**: Enum members must be UpperCamelCase or ALL_CAPS

## Details

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
```

### Bad

```systemverilog
typedef enum int unsigned {
  off,
  on
} state_e;
```
