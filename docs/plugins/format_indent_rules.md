# Format indent rules

- **Scripts**: `plugins/format.*.raw.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Shared Helpers**: `plugins/lib/format_indent_ruleset.py`
- **Rules**:
  - ``format.indent_multiple_of_two`` (warning): Enforce indentation in multiples of two spaces
  - ``format.preproc_left_align`` (warning): Left-align preprocessor directives
  - ``format.line_continuation_right`` (warning): Require `\` line continuations to be the last character

## Rule Details

### `format.indent_multiple_of_two`
#### Trigger
Computes indentation width after stripping tabs; flags lines with an odd number of spaces.
#### Message
`` indentation should be multiples of 2 spaces ``
#### Remediation
Replace tabs with spaces and keep indentation at two-space steps.
#### Good

```systemverilog
module foo;
  logic data_q;
endmodule
```

#### Bad

```systemverilog
module foo;
   logic  data_q;  // uneven indentation
endmodule
```

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

### `format.line_continuation_right`
#### Trigger
Checks lines containing `\` and warns when characters follow the backslash.
#### Message
`` line continuation \ must be last character ``
#### Remediation
Ensure the backslash is the final character—move comments to the next line.
#### Good

```systemverilog
`define INCR(value) \
  (value + 1)
```

#### Bad

```systemverilog
`define INCR(value) \ // comment after backslash
  (value + 1)
```
