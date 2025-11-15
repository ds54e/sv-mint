# lang.default_nettype_none

- **Script**: `plugins/lang.default_nettype_none.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Shared Helpers**: `plugins/lib/default_nettype_ruleset.py`
- **Summary**: The first `default_nettype` must set the value to `none`

## Details

Even when a directive exists, it must explicitly set the value to `none`. Any other value (`wire`, `tri`, etc.) raises a violation so the toolchain doesn’t fall back to implicit nets mid-file.
### Good

```systemverilog
`default_nettype none
```

### Bad

```systemverilog
`default_nettype wire  // must start at none
```
