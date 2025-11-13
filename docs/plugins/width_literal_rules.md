# width_literal_rules.py

- **Script**: `plugins/width_literal_rules.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Rule**:
  | Rule ID | Severity | Summary |
  | --- | --- | --- |
  | `width.unsized_base_literal` | warning | Ban base literals without explicit widths |

## Rule Details

### `width.unsized_base_literal`
- **Trigger**: Regex `(?<![0-9_])'(b|B|d|D|h|H|o|O)` finds `'hFF`-style literals lacking a width.
- **Message**: `` base literal must include explicit width (e.g. 8'hFF) ``
- **Remediation**: Add widths (`8'h`, `4'd`, etc.) to every base literal.
- **LowRISC Reference**: Unsized base literals are forbidden; always spell out widths.
- **Additional Tips**: Use underscores for readability (`32'hDEAD_BEEF`) and move constants into `localparam` for reuse.
