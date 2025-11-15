# format_case_colon_spacing.py

- **Script**: `plugins/format.case_colon_spacing.cst.py`
- **Stage**: `cst`
- **Key Inputs**: `text`, `cst_ir.pp_text`, `line_starts`
- **Shared Helpers**: `plugins/lib/format_spacing_ruleset.py`
- **Rule**:
  - ``format.case_colon_spacing`` (warning): Forbid whitespace before `:` in case labels

## Rule Details

### `format.case_colon_spacing`
#### Trigger
In CST mode, inspect `CaseItem` tokens and flag any whitespace that appears before `:`.
#### Message
`` case item must not have whitespace before ':' ``
#### Remediation
Format labels as `LABEL:` so the colon touches the label; follow-on spacing is handled by `format.case_colon_after`.
#### Notes
Only case labels are analyzed (not enums or `localparam`). When adding comments, keep `LABEL: // comment` ordering so stray spaces do not sneak in front of the colon.
#### Good

```systemverilog
unique case (state_q)
  IDLE:   data_d = IDLE_NEXT;
  DONE:   data_d = DONE_NEXT;
  default: data_d = state_q;
endcase
```

#### Bad

```systemverilog
unique case (state_q)
  IDLE : data_d = IDLE_NEXT;  // space before colon
  DONE:   data_d = DONE_NEXT;
  default: data_d = state_q;
endcase
```
