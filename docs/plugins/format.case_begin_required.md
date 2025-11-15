# format.case_begin_required

- **Stage**: `cst` (`mode = inline`)
- **Key Inputs**: `cst_ir.tokens`, `tok_kind_table`, `line_starts`
- **Summary**: Require each `case` item to wrap statements in `begin/end`

## Details

### Trigger
For each `CaseStatement`, checks the first non-comment token after the `colon`. If it is not `begin`, the rule fires (even for single statements).
### Message
`` case item should wrap statements in begin/end ``
### Remediation
Add `begin ... end` blocks to every case item so the first token after `:` is always `begin`.
### Notes
Single statements that already start with `begin` are skipped; the policy applies equally to `unique case` and `case inside`. The implementation does not count statements inside the item—any label without an immediate `begin` fails the rule.
### Good

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

### Bad

```systemverilog
case (state_q)
  START: ready_o = 1'b1;
          state_d = RUN;  // multiple statements without begin/end
endcase
```

### Additional Tips
Adding labels like `begin : start_state` helps readability and avoids confusion when inserting inline comments.
