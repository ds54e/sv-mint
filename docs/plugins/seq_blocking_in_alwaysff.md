# seq_blocking_in_alwaysff.py

- **Script**: `plugins/seq_blocking_in_alwaysff.py`
- **Stage**: `cst`
- **Key Inputs**: `cst_ir.tokens`, `tok_kind_table`, `line_starts`, `pp_text`
- **Rule**:
  | Rule ID | Severity | Summary |
  | --- | --- | --- |
  | `seq.blocking_in_alwaysff` | warning | Ban blocking `=` assignments inside `always_ff` |

## Rule Details

### `seq.blocking_in_alwaysff`
- **Trigger**: Identifies `always_ff` regions and warns whenever an `op_eq` (or fallback regex `=`) token appears inside.
- **Message**: `` blocking '=' inside always_ff ``
- **Remediation**: Use non-blocking `<=` for sequential logic or refactor the assignment into combinational logic.
- **Notes**: Falls back to text scanning when token data is unavailable.
