# format.preproc_left_align

- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Shared Helpers**: `plugins/lib/format_indent_ruleset.py`
- **Summary**: Left-align preprocessor directives

## Details

### Trigger
Finds `define/ifdef/ifndef/endif` directives that start with whitespace.
### Message
`` preprocessor directives must be left aligned ``
### Remediation
Remove leading whitespace so directives start in column 1, regardless of nesting depth.
### Good

```systemverilog
`ifdef FOO
logic bar;
`endif
```

### Bad

```systemverilog
  `ifdef FOO  // directive indented
logic bar;
  `endif
```
