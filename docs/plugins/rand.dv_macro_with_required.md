# rand.dv_macro_with_required

- **Script**: `plugins/rand.dv_macro_with_required.raw.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`, `path`
- **Shared Helpers**: `plugins/lib/dv_text_ruleset.py`
- **Summary**: Require the `_WITH` DV macros when constraints are present

## Details

### Trigger
Detects `randomize() with { ... }` blocks not already wrapped by `_WITH` macros.
### Message
`` use DV_CHECK_*_WITH_FATAL macros when constraints are present ``
### Remediation
Switch to `DV_CHECK_RANDOMIZE_WITH_FATAL`, `DV_CHECK_STD_RANDOMIZE_WITH_FATAL`, etc.
### Good

```systemverilog
DV_CHECK_RANDOMIZE_WITH_FATAL(req.randomize() with { kind inside {READ}; });
```

### Bad

```systemverilog
req.randomize() with { kind inside {READ}; };
```
