# naming_clk_prefix.py

- **Script**: `plugins/naming.clk_prefix.ast.py`
- **Stage**: `ast`
- **Key Inputs**: `decls`, `symbols`, `ports`
- **Shared Helpers**: `plugins/lib/naming_ruleset.py`
- **Rule**:
  - ``naming.clk_prefix`` (warning): Clock names must start with `clk`

## Rule Details

### `naming.clk_prefix`
#### Trigger
Requires clock ports to start with `clk`.
#### Message
`` clock port <name> must start with 'clk' ``
#### Remediation
Rename to `clk_<domain>_<suffix>`.
#### Good

```systemverilog
input logic clk_core_i;
```

#### Bad

```systemverilog
input logic core_clk_i;
```
