# format_line_length.py

- **Script**: `plugins/format_line_length.py`
- **Stage**: `raw_text`
- **Key Inputs**: LF-normalized `text`
- **Rule**:
  | Rule ID | Severity | Summary |
  | --- | --- | --- |
  | `format.line_length` | warning | Flag lines longer than 100 columns |

## Rule Details

### `format.line_length`
- **Trigger**: Measures each line and reports those exceeding `MAX_COLUMNS = 100`, pointing at column 101+.
- **Message**: `` line exceeds 100 columns (118) ``
- **Remediation**: Break long expressions, introduce temporaries, or wrap comments to stay within 100 columns.
- **Notes**: Threshold is fixed in code; tweak severity via `ruleset.override` if needed.
- **LowRISC Reference**: The style guide caps SystemVerilog lines at 100 characters, including doc comments.
