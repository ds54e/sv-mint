# format_preproc_left_align.py

- **Script**: `plugins/format.preproc_left_align.raw.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Shared Helpers**: `plugins/lib/format_indent_ruleset.py`
- **Rule**:
  - ``format.preproc_left_align`` (warning): Left-align preprocessor directives

## Rule Details

### `format.preproc_left_align`
#### Trigger
Finds `define/ifdef/ifndef/endif` directives that start with whitespace.
#### Message
`` preprocessor directives must be left aligned ``
#### Remediation
Remove leading whitespace so directives start in column 1, regardless of nesting depth.
#### Good

```systemverilog
`ifdef FOO
logic bar;
`endif
```

#### Bad

```systemverilog
  `ifdef FOO  // directive indented
logic bar;
  `endif
```
