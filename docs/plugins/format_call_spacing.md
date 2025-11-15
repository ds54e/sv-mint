# format_call_spacing.py

- **Script**: `plugins/format.call_spacing.raw.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`, `cst_ir.pp_text`, `line_starts`
- **Shared Helpers**: `plugins/lib/format_spacing_ruleset.py`
- **Summary**: Disallow spaces between function/task names and `(`

## Details

### Trigger
Detects `foo (` in call sites (declarations like `function foo (` are ignored).
### Message
`` function or task call must not have space before '(' ``
### Remediation
Use `foo(`.
### Notes
For multiline argument lists, break right after `(` to avoid other spacing rules.
### Good

```systemverilog
foo(a, b);
```

### Bad

```systemverilog
foo (a, b);
```
