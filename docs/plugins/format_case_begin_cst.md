# format_case_begin_cst.py

- **Script**: `plugins/format_case_begin_cst.py`
- **Stage**: `cst` (`mode = inline`)
- **Key Inputs**: `cst_ir.tokens`, `tok_kind_table`, `line_starts`
- **Rule**:
  | Rule ID | Severity | Summary |
  | --- | --- | --- |
  | `format.case_begin_required` | warning | Require each `case` item to wrap statements in `begin/end` |

## Rule Details

### `format.case_begin_required`
- **Trigger**: For each `CaseStatement`, checks the first non-comment token after the `colon`. If it is not `begin`, the rule fires.
- **Message**: `` case item should wrap statements in begin/end ``
- **Remediation**: Add `begin ... end` blocks to each case item that holds multiple statements (and consider doing so even for single statements for future-proofing).
- **Notes**: Single statements that already start with `begin` are skipped; the policy applies equally to `unique case` and `case inside`.
- **Good**:

```systemverilog
unique case (state_q)
  IDLE: begin
    ready_o = 1'b1;
    state_d = START;
  end
  default: begin
    ready_o = 1'b0;
  end
endcase
```

- **Bad**:

```systemverilog
case (state_q)
  START: ready_o = 1'b1;
          state_d = RUN;  // multiple statements without begin/end
endcase
```

- **Additional Tips**: Adding labels like `begin : start_state` helps readability and avoids confusion when inserting inline comments.
