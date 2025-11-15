# format_spacing.py

- **Script**: `plugins/format_spacing.py`
- **Stages**: `raw_text` and `cst`
- **Key Inputs**: `text`, `cst_ir.pp_text`, `line_starts`
- **Rules**:
  - ``format.comma_space`` (warning): Require a space after commas
  - ``format.call_spacing`` (warning): Disallow spaces between function/task names and `(`
  - ``format.macro_spacing`` (warning): Disallow spaces between macro names and `(`
  - ``format.case_colon_spacing`` (warning): Forbid whitespace before `:` in case labels
  - ``format.case_colon_after`` (warning): Require whitespace after `:` in case labels

## Rule Details

### `format.comma_space`
- **Trigger**: Regex `,(?!\s)` finds commas not followed by whitespace.
- **Message**: `` missing space after comma ``
- **Remediation**: Separate arguments and concatenations with `, ` for readability.
- **Notes**: Applies to macro arguments as well. If packed literals require different spacing, adjust the script locally or disable the rule via its `[[rule]]` entry.

### `format.call_spacing`
- **Trigger**: Detects `foo (` in call sites (declarations like `function foo (` are ignored).
- **Message**: `` function or task call must not have space before '(' ``
- **Remediation**: Use `foo(`.
- **Notes**: For multiline argument lists, break right after `(` to avoid other spacing rules.

### `format.macro_spacing`
- **Trigger**: Flags macro invocations with spaces before `(`.
- **Message**: `` macro invocation must not have space before '(' ``
- **Remediation**: Use `` `MY_MACRO(`` syntax consistently.

### `format.case_colon_spacing` / `format.case_colon_after`
- **Trigger**: In CST mode, inspect `CaseItem` tokens and check whitespace around `:`.
- **Messages**:
  - `` case item must not have whitespace before ':' ``
  - `` case item must have space after ':' ``
- **Remediation**: Format labels as `LABEL: statement;` with no space before and exactly one space after the colon.
- **Notes**: Only case labels are analyzed (not enums or `localparam`). When adding comments, keep `LABEL: // comment` ordering to satisfy both rules.
- **Good**:

```systemverilog
foo(a, b, c);
`MY_MACRO(a, b)
unique case (state_q)
  IDLE: data_d = IDLE_NEXT;
  DONE: data_d = DONE_NEXT;
  default: data_d = state_q;
endcase
```

- **Bad**:

```systemverilog
foo (a,b,c);
`MY_MACRO (a,b)
unique case (state_q)
  IDLE :data_d = IDLE_NEXT;
  DONE:begin
    data_d = DONE_NEXT;
  end
endcase
```
