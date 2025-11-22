# typedef_names_lower_snake_t

## Script
- `plugins/typedef_names_lower_snake_t.cst.py`

## Description
- Typedef names (non-enum) must be lower_snake_case and end with `_t`
- Why: `_t` snake-case names clearly mark typedefs versus signals.
## Good

```systemverilog
typedef logic [3:0] my_type_t;
```

## Bad

```systemverilog
typedef logic [3:0] MyType_t;
typedef logic [3:0] MY_TYPE;
```
