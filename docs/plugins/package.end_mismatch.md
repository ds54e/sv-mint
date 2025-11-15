# package.end_mismatch

- **Script**: `plugins/package.end_mismatch.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Shared Helpers**: `plugins/lib/package_ruleset.py`
- **Summary**: Ensure `endpackage : name` matches the package name

## Details

### Trigger
Compares `endpackage : label` with the original `package name` and warns on mismatches.
### Message
`` endpackage label bar does not match package foo ``
### Remediation
Fix the label or regenerate the file with consistent templates.
### Good

```systemverilog
package foo_pkg;
endpackage : foo_pkg
```

### Bad

```systemverilog
package foo_pkg;
endpackage : bar_pkg
```
