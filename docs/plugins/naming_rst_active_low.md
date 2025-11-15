# naming_rst_active_low.py

- **Script**: `plugins/naming.rst_active_low.ast.py`
- **Stage**: `ast`
- **Key Inputs**: `decls`, `symbols`, `ports`
- **Shared Helpers**: `plugins/lib/naming_ruleset.py`
- **Rule**:
  - ``naming.rst_active_low`` (warning): Reset names must end with `_n` (or `_ni/_no/_nio`)

## Rule Details

### `naming.rst_active_low`
#### Trigger
Ensures reset names end in `_n`, `_ni`, `_no`, or `_nio`.
#### Message
`` reset <name> must use active-low suffix `_n` ``
#### Remediation
Rename resets to `rst_ni`, `rst_no`, etc.
#### Good

```systemverilog
input logic rst_ni;
```

#### Bad

```systemverilog
input logic rst_i;
```
