# header.missing_comment

- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Shared Helpers**: `plugins/lib/header_comment_ruleset.py`
- **Summary**: Require a header comment within the first five lines

## Details

### Trigger
If the first five lines contain no `//` comment, the rule fires.
### Message
`` file header should include descriptive comment ``
### Remediation
Summarize module purpose, contacts, or key context at the top of the file.
### Good

```systemverilog
// Control logic for entropy distribution network.
module entropy_ctrl (...);
```

### Bad

```systemverilog
module entropy_ctrl (...);
```

### Additional Tips
Mention module names, roles, and dependency URLs when useful. Generators should emit these comments automatically.
