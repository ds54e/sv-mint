# macros_not_unused

- **Script**: `plugins/macros_not_unused.raw.py`
- **Stage**: `raw_text`
- **Summary**: Warn when a macro is defined but never used

## Details

### Message
`` macro `<name>` is defined but never used ``

### Remediation
Remove the unused macro or use it where intended. For one-off local helpers, prefer inlining or deleting the dead definition.

### Notes
This rule operates on pre-preprocessor text, so comments and disabled code (`ifdef` branches) still count as “usage” if they include ``NAME`.
