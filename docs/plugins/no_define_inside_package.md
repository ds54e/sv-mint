# no_define_inside_package

- **Script**: `plugins/no_define_inside_package.raw.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Shared Helpers**: `plugins/lib/package_ruleset.py`
- **Summary**: Forbid `` `define`` inside packages

## Details

### Message
`` prefer parameters over `define NAME inside package ``
### Remediation
Publish constants via `parameter` or `localparam` instead of macros.
### Good

```systemverilog
package foo_pkg;
  parameter bit EnableFoo = 1'b1;
endpackage
```

### Bad

```systemverilog
package foo_pkg;
  `define ENABLE_FOO 1  // macros inside packages are banned
endpackage
```
