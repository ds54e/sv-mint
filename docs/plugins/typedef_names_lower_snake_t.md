# typedef_names_lower_snake_t

- **Script**: `plugins/typedef_names_lower_snake_t.cst.py`
- **Stage**: `cst`
- **Key Inputs**: `cst_ir.tokens`, `line_starts`, `pp_text`
- **Summary**: Typedef names (non-enum) must be lower_snake_case and end with `_t`

## Details

### Message
`` typedef names should use lower_snake_case and end with _t: <name> ``

### Remediation
Rename typedefs such as `data_width_t`, `foo_bar_t`, etc.

### Good

```systemverilog
typedef logic [3:0] my_type_t;
```systemverilog

### Bad

```systemverilog
typedef logic [3:0] MyType_t;
typedef logic [3:0] MY_TYPE;
```systemverilog