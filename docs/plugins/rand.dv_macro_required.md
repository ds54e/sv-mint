# rand.dv_macro_required

- **Script**: `plugins/rand.dv_macro_required.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`, `path`
- **Shared Helpers**: `plugins/lib/dv_text_ruleset.py`
- **Summary**: Enforce `DV_CHECK_*RANDOMIZE*` macros instead of raw `randomize()`

## Details

### Trigger
Looks for bare `randomize()` or `std::randomize()` calls.
### Message
`` use DV_CHECK_*RANDOMIZE* macros instead of raw randomize() ``
### Remediation
Wrap every randomization call with `DV_CHECK_RANDOMIZE_FATAL`, `DV_CHECK_STD_RANDOMIZE_FATAL`, or `DV_CHECK_MEMBER_RANDOMIZE_FATAL`.
### Good

```systemverilog
DV_CHECK_RANDOMIZE_FATAL(req.randomize());
```

### Bad

```systemverilog
req.randomize();  // missing DV_CHECK_* wrapper
```
