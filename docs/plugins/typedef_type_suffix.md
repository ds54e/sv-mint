# typedef_type_suffix.py

- **Script**: `plugins/typedef.type_suffix.raw.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Shared Helpers**: `plugins/lib/typedef_naming_ruleset.py`
- **Summary**: Require other typedef names to end with `_t`

## Details

### Trigger
Non-enum typedef names lacking `_t`.
### Message
`` typedef names should end with _t: data ``
### Remediation
Append `_t`, e.g., `data_t`.
### Additional Tips
When exporting typedefs from packages, keep the `_t` suffix for downstream consistency.
### Good

```systemverilog
typedef struct packed {
  logic valid;
  logic [31:0] data;
} payload_t;
```

### Bad

```systemverilog
typedef struct packed {
  logic valid;
  logic [31:0] data;
} payload;
```
