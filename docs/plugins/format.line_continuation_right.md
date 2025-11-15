# format.line_continuation_right

- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Shared Helpers**: `plugins/lib/format_indent_ruleset.py`
- **Summary**: Require `\` line continuations to be the last character

## Details

### Trigger
Checks lines containing `\` and warns when characters follow the backslash.
### Message
`` line continuation \ must be last character ``
### Remediation
Ensure the backslash is the final character—move comments to the next line.
### Good

```systemverilog
`define INCR(value) \
  (value + 1)
```

### Bad

```systemverilog
`define INCR(value) \ // comment after backslash
  (value + 1)
```
