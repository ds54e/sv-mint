# macros_not_unused

- **Script**: `plugins/macros_not_unused.raw.py`
- **Stage**: `raw_text`
- **Summary**: Warn when a macro is defined but never used

## Details

### Message
`` macro `<name>` is defined but never used ``

### Remediation
Remove the unused macro or use it where intended. For one-off local helpers, prefer inlining or deleting the dead definition.

### Limitations
- A comment containing the words `used` or `reserved` (case-insensitive) in the macro definition block suppresses this warning.
