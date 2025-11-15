# format_line_continuation_right.py

- **Script**: `plugins/format.line_continuation_right.raw.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Shared Helpers**: `plugins/lib/format_indent_ruleset.py`
- **Rule**:
  - ``format.line_continuation_right`` (warning): Require `\` line continuations to be the last character

## Rule Details

### `format.line_continuation_right`
#### Trigger
Checks lines containing `\` and warns when characters follow the backslash.
#### Message
`` line continuation \ must be last character ``
#### Remediation
Ensure the backslash is the final character—move comments to the next line.
#### Good

```systemverilog
`define INCR(value) \
  (value + 1)
```

#### Bad

```systemverilog
`define INCR(value) \ // comment after backslash
  (value + 1)
```
