# package_rules.py

- **Script**: `plugins/package_rules.py`
- **Stage**: `raw_text`
- **Key Inputs**: `text`
- **Rules**:
  - ``package.multiple`` (warning): Limit each file to a single `package`
  - ``package.missing_end`` (warning): Require `endpackage` when a package is declared
  - ``package.end_mismatch`` (warning): Ensure `endpackage : name` matches the package name
  - ``package.define_in_package`` (warning): Forbid `` `define`` inside packages

## Rule Details

### `package.multiple`
#### Trigger
Counts `package` keywords; if more than one appears, the rule reports the first occurrence.
#### Message
`` multiple package declarations in single file (pkg_name) ``
#### Remediation
Split packages into separate files or rename them.
#### Additional Tips
If packages must share a file with interfaces, configure exceptions or separate the sources.

### `package.missing_end`
#### Trigger
Detects `package` without a matching `endpackage`.
#### Message
`` package foo missing endpackage ``
#### Remediation
Add `endpackage : foo`.
#### Notes
Do not wrap `endpackage` in conditionals; place `ifdef` blocks inside the package body.

### `package.end_mismatch`
#### Trigger
Compares `endpackage : label` with the original `package name` and warns on mismatches.
#### Message
`` endpackage label bar does not match package foo ``
#### Remediation
Fix the label or regenerate the file with consistent templates.

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
  parameter int DataWidth = 32;
endpackage : foo_pkg
```

#### Bad

```systemverilog
package foo_pkg;
  parameter int DataWidth = 32;
endpackage : bar_pkg  // mismatched end label and missing second package guard

package extra_pkg;  // multiple packages per file
  `define ENABLE_FOO 1  // macro defined inside package
endpackage
```
