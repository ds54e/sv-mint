# format_comma_space.py

- **Script**: `plugins/format.comma_space.raw.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`, `cst_ir.pp_text`, `line_starts`
- **Shared Helpers**: `plugins/lib/format_spacing_ruleset.py`
- **Rule**:
  - ``format.comma_space`` (warning): Require a space after commas

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
