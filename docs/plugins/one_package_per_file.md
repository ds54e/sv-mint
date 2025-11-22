# one_package_per_file

- **Script**: `plugins/one_package_per_file.raw.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Summary**: Limit each file to a single `package`

## Details

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
