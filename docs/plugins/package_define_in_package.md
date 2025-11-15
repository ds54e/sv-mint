# package_define_in_package.py

- **Script**: `plugins/package.define_in_package.raw.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Shared Helpers**: `plugins/lib/package_ruleset.py`
- **Rule**:
  - ``package.define_in_package`` (warning): Forbid `` `define`` inside packages

## Rule Details

### `package.define_in_package`
#### Trigger
Searches the package body for `` `define`` tokens that do not start with `_`.
#### Message
`` prefer parameters over `define NAME inside package ``
#### Remediation
Publish constants via `parameter` or `localparam` instead of macros.
#### Additional Tips
Transition legacy macros to `localparam` and consume them through `import foo_pkg::*;`.
#### Good

```systemverilog
package foo_pkg;
  parameter bit EnableFoo = 1'b1;
endpackage
```

#### Bad

```systemverilog
package foo_pkg;
  `define ENABLE_FOO 1  // macros inside packages are banned
endpackage
```
