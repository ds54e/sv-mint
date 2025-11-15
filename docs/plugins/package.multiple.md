# package.multiple

- **Script**: `plugins/package.multiple.raw.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Shared Helpers**: `plugins/lib/package_ruleset.py`
- **Summary**: Limit each file to a single `package`

## Details

### Trigger
Counts `package` keywords; if more than one appears, the rule reports the first occurrence.
### Message
`` multiple package declarations in single file (pkg_name) ``
### Remediation
Split packages into separate files or rename them.
### Good

```systemverilog
package foo_pkg;
endpackage : foo_pkg
```

### Bad

```systemverilog
package foo_pkg;
endpackage

package bar_pkg;  // second package in same file
endpackage
```
