# package_missing_end.py

- **Script**: `plugins/package.missing_end.raw.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Shared Helpers**: `plugins/lib/package_ruleset.py`
- **Summary**: Require `endpackage` when a package is declared

## Details

### Trigger
Detects `package` without a matching `endpackage`.
### Message
`` package foo missing endpackage ``
### Remediation
Add `endpackage : foo`.
### Notes
Do not wrap `endpackage` in conditionals; place `ifdef` blocks inside the package body.
### Good

```systemverilog
package foo_pkg;
  // declarations
endpackage : foo_pkg
```

### Bad

```systemverilog
package foo_pkg;
  // declarations
// missing endpackage
```
