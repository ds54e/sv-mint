# macro_no_unused_macro

- **Script**: `plugins/macro_no_unused_macro.raw.py`
- **Stage**: `raw_text`
- **Summary**: Warn when a macro is defined but never used

## Details

### Trigger
Scans `raw_text` for ``define NAME``, collects macro invocations (``NAME` anywhere in the source), and reports any defined macro that is never used.
- If a macro is `undef`’d, it is treated as used and not reported.

### Message
`` macro `<name>` is defined but never used ``

### Remediation
Remove the unused macro or use it where intended. For one-off local helpers, prefer inlining or deleting the dead definition.

### Notes
This rule operates on pre-preprocessor text, so comments and disabled code (`ifdef` branches) still count as “usage” if they include ``NAME`.
