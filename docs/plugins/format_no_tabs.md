# format.no_tabs.raw.py

- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Shared Helpers**: `plugins/lib/format_text_ruleset.py`
- **Summary**: Reject tab characters

## Details

### Trigger
Emits a violation for every tab (`\t`) encountered.
### Message
`` tab character detected ``
### Remediation
Replace tabs with spaces and follow the widths enforced by `format_indent_rules`.
### Good

```systemverilog
logic ready;
```

### Bad

```systemverilog
	logic ready;
```

- Tabs at the start of the line shift alignment between tools.

### Notes
Pair this with `.editorconfig` `indent_style = space`. If you absolutely must allow tabs (e.g., when linting legacy IP), disable the rule via its `[[rule]]` entry and re-enable it once the migration is complete.
