# format.macro_spacing

- **Stage**: `raw_text`
- **Key Inputs**: `text`, `cst_ir.pp_text`, `line_starts`
- **Shared Helpers**: `plugins/lib/format_spacing_ruleset.py`
- **Summary**: Disallow spaces between macro names and `(`

## Details

### Trigger
Flags macro invocations with spaces before `(`.
### Message
`` macro invocation must not have space before '(' ``
### Remediation
Use `` `MY_MACRO(`` syntax consistently.
### Good

```systemverilog
`MY_MACRO(a, b)
```

### Bad

```systemverilog
`MY_MACRO (a, b)
```
