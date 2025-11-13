# case_default_required.py

- **Script**: `plugins/case_default_required.py`
- **Stage**: `cst` (`mode = inline`)
- **Key Inputs**: `cst_ir.tokens`, `line_starts`, `pp_text`
- **Rule**:
  | Rule ID | Severity | Summary |
  | --- | --- | --- |
  | `case.missing_default` | warning | Warn when a `case` statement lacks a `default` label |

## Rule Details

### `case.missing_default`
- **Trigger**: Walks each CST `CaseStatement`; if no `default` token appears, the rule reports the first token location.
- **Message**: `` case statement must include a default item ``
- **Remediation**: Add a `default` branch unless you can prove completeness with `unique case`. Even for intentional fall-through, prefer `default: <noop>;`.
- **Notes**: The pass inspects preprocessed `pp_text`, so macros that expand to `default` must emit the token after preprocessing.
- **LowRISC Reference**: The Case Statements section requires `default` even for `unique case` so abnormal values fall into an explicit handler like `state_d = state_q;`.
- **Good**:

```systemverilog
unique case (state_q)
  IDLE:   state_d = START;
  START:  state_d = DONE;
  default: state_d = IDLE;  // handle unexpected states
endcase
```

- **Bad**:

```systemverilog
case (opcode_i)
  4'h0: alu_d = ADD;
  4'h1: alu_d = SUB;
endcase  // no default, unknown values pass silently
```

- **Additional Tips**: When wrapping `default` in `begin/end`, keep the colon directly after the token. Align macro-generated `default` blocks with the surrounding case body to reduce review slips.
