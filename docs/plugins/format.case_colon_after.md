# format.case_colon_after

- **Script**: `plugins/format.case_colon_after.py`
- **Stage**: `cst`
- **Key Inputs**: `text`, `cst_ir.pp_text`, `line_starts`
- **Shared Helpers**: `plugins/lib/format_spacing_ruleset.py`
- **Summary**: Require whitespace after `:` in case labels

## Details

### Trigger
In CST mode, inspect `CaseItem` tokens and flag missing whitespace after `:`.
### Message
`` case item must have space after ':' ``
### Remediation
Ensure `LABEL: statement;` includes a space between the colon and the first statement; pair with `format.case_colon_spacing` to cover both sides.
### Notes
Only case labels are analyzed (not enums or `localparam`). When adding comments, keep `LABEL: // comment` ordering so a single space separates the colon from text.
### Good

```systemverilog
unique case (state_q)
  IDLE:   data_d = IDLE_NEXT;
  DONE:   data_d = DONE_NEXT;
  default: data_d = state_q;
endcase
```

### Bad

```systemverilog
unique case (state_q)
  IDLE:   data_d = IDLE_NEXT;
  DONE:begin                 // no space after colon
    data_d = DONE_NEXT;
  end
endcase
```
