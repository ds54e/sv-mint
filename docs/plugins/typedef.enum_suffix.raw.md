# typedef.enum_suffix.raw.py

- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Shared Helpers**: `plugins/lib/typedef_naming_ruleset.py`
- **Summary**: Require `typedef enum` names to end with `_e`

## Details

### Trigger
Matches `typedef enum { ... } name;` constructs whose `name` does not end with `_e`.
### Message
`` enum types should end with _e: state ``
### Remediation
Rename to `state_e`, etc.
### Additional Tips
Do not use `_t` for enums; that conflicts with the struct rule below.
### Good

```systemverilog
typedef enum logic [1:0] {
  IDLE,
  BUSY
} state_e;
```

### Bad

```systemverilog
typedef enum logic [1:0] {
  IDLE,
  BUSY
} state;
```
