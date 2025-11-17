# format.final_newline

- **Script**: `plugins/format.final_newline.raw.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Shared Helpers**: `plugins/lib/format_text_ruleset.py`
- **Summary**: Require a trailing newline

## Details

### Trigger
Warns when the file does not end with `\n`.
### Message
`` file must end with newline ``
### Remediation
Insert a newline after the last line.
### Good

```systemverilog
module foo;
endmodule

```

### Bad

```systemverilog
module foo;
endmodule
```
### Notes
Git adds `\ No newline at end of file` to diffs; this rule catches the issue before CI noise appears.
