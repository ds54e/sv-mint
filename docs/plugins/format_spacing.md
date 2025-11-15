# Format spacing rules

- **Scripts**: `plugins/<rule_id>.(raw|cst).py`
- **Stages**: `raw_text` (`format.comma_space`, `format.call_spacing`, `format.macro_spacing`), `cst` (`format.case_colon_*`)
- **Key Inputs**: `text`, `cst_ir.pp_text`, `line_starts`
- **Shared Helpers**: `plugins/lib/format_spacing_ruleset.py`
- **Rules**:
  - ``format.comma_space`` (warning): Require a space after commas
  - ``format.call_spacing`` (warning): Disallow spaces between function/task names and `(`
  - ``format.macro_spacing`` (warning): Disallow spaces between macro names and `(`
  - ``format.case_colon_spacing`` (warning): Forbid whitespace before `:` in case labels
  - ``format.case_colon_after`` (warning): Require whitespace after `:` in case labels

## Rule Details

### `format.comma_space`
#### Trigger
Regex `,(?!\s)` finds commas not followed by whitespace.
#### Message
`` missing space after comma ``
#### Remediation
Separate arguments and concatenations with `, ` for readability.
#### Notes
Applies to macro arguments as well. If packed literals require different spacing, adjust the script locally or disable the rule via its `[[rule]]` entry.
#### Good

```systemverilog
foo(a, b, c);
```

#### Bad

```systemverilog
foo(a,b,c);
```

### `format.call_spacing`
#### Trigger
Detects `foo (` in call sites (declarations like `function foo (` are ignored).
#### Message
`` function or task call must not have space before '(' ``
#### Remediation
Use `foo(`.
#### Notes
For multiline argument lists, break right after `(` to avoid other spacing rules.
#### Good

```systemverilog
foo(a, b);
```

#### Bad

```systemverilog
foo (a, b);
```

### `format.macro_spacing`
#### Trigger
Flags macro invocations with spaces before `(`.
#### Message
`` macro invocation must not have space before '(' ``
#### Remediation
Use `` `MY_MACRO(`` syntax consistently.
#### Good

```systemverilog
`MY_MACRO(a, b)
```

#### Bad

```systemverilog
`MY_MACRO (a, b)
```

### `format.case_colon_spacing` / `format.case_colon_after`
#### Trigger
In CST mode, inspect `CaseItem` tokens and check whitespace around `:`.
- **Messages**:
  - `` case item must not have whitespace before ':' ``
  - `` case item must have space after ':' ``
#### Remediation
Format labels as `LABEL: statement;` with no space before and exactly one space after the colon.
#### Notes
Only case labels are analyzed (not enums or `localparam`). When adding comments, keep `LABEL: // comment` ordering to satisfy both rules.
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
  IDLE :data_d = IDLE_NEXT;  // space before colon
  DONE:begin                // no space after colon
    data_d = DONE_NEXT;
  end
endcase
```
