# format.comma_space

- **Script**: `plugins/format.comma_space.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`, `cst_ir.pp_text`, `line_starts`
- **Shared Helpers**: `plugins/lib/format_spacing_ruleset.py`
- **Summary**: Require a space after commas

## Details

### Trigger
Regex `,(?!\s)` finds commas not followed by whitespace.
### Message
`` missing space after comma ``
### Remediation
Separate arguments and concatenations with `, ` for readability.
### Notes
Applies to macro arguments as well. If packed literals require different spacing, adjust the script locally or disable the rule via its `[[rule]]` entry.
### Good

```systemverilog
foo(a, b, c);
```

### Bad

```systemverilog
foo(a,b,c);
```
