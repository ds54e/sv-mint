# naming_suffix_order.py

- **Script**: `plugins/naming.suffix_order.ast.py`
- **Stage**: `ast`
- **Key Inputs**: `decls`, `symbols`, `ports`
- **Shared Helpers**: `plugins/lib/naming_ruleset.py`
- **Rule**:
  - ``naming.suffix_order`` (warning): Enforce `_ni/_no/_nio` suffix ordering

## Rule Details

### `naming.suffix_order`
#### Trigger
Catches split suffixes like `_n_i` or `_n_o`.
#### Message
`` combine reset and direction suffixes (e.g. rst_ni) ``
#### Remediation
Merge `_n` with `_i/_o/_io` to form `_ni/_no/_nio`.
#### Good

```systemverilog
logic rst_ni;
```

#### Bad

```systemverilog
logic rst_n_i;
```
