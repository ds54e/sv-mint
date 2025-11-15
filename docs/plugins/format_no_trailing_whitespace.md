# format_no_trailing_whitespace.py

- **Script**: `plugins/format.no_trailing_whitespace.raw.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Shared Helpers**: `plugins/lib/format_text_ruleset.py`
- **Summary**: Flag trailing whitespace

## Details

### Trigger
Reverse scans each line and flags trailing spaces or tabs.
### Message
`` trailing whitespace at line end ``
### Remediation
Trim on save or rely on editor hooks.
### Good

```systemverilog
assign ready_o = valid_i;
```

### Bad

```systemverilog
assign ready_o = valid_i;␠
```

### Notes
sv-mint analyzes LF-normalized text, so CRLF mixes still produce correct columns. Consider the `trailing-whitespace` pre-commit hook to catch violations before CI.
